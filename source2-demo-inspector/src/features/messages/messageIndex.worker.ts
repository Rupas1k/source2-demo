import { expose, transfer } from "comlink";
import { isFullPacket } from "./messageIndex";
import { gameEventDefinitions } from "./packetGameEvents";
import type { FiredEvent } from "../../replay/readGameEvents";
import { packetTableRecords, snapshotTables } from "./packetStringTables";
import initInspector, {
  MessageInspector,
} from "../../generated/wasm/inspector";
import { sortRows } from "../../workbench/tableSort";
import type {
  Entry,
  IndexedEntry,
  MessageIndexApi,
  MessageIndexCalls,
  Query,
} from "./messageIndex";

let inspector: MessageInspector;
let opening: Promise<void>;
let commands: Entry[] | undefined;
let lastTerm: string | undefined;
let matches: Entry[] = [];
let rows: IndexedEntry[] = [];
let groups: Map<number, { parent: Entry; children: Entry[] }> | undefined;
let firstAtTick = new Map<number, Entry>();
let points = new Uint32Array();
let revision = 0;
let fired: Map<string, FiredEvent> | undefined;

function firedEvents() {
  return (fired ??= new Map(
    (JSON.parse(inspector.firedEvents()) as FiredEvent[]).map((event) => [
      event.command + "/" + event.child,
      event,
    ]),
  ));
}

function counts(entries: Entry[]) {
  const byTick = new Map<number, number>();
  for (const { tick } of entries) byTick.set(tick, (byTick.get(tick) ?? 0) + 1);
  return Uint32Array.from(Array.from(byTick).flat());
}

function query({
  term,
  wide,
  sort,
  packetsOpen,
  fullPacketsOpen,
  revealedPacket,
}: Query) {
  if (lastTerm !== term) {
    matches = JSON.parse(inspector.search(term));
    firstAtTick = new Map();
    for (const entry of matches)
      if (!firstAtTick.has(entry.tick)) firstAtTick.set(entry.tick, entry);
    points = term ? counts(matches) : new Uint32Array();
    lastTerm = term;
    groups = undefined;
  }
  rows = [];
  if (wide) {
    if (!groups) {
      groups = new Map();
      for (const entry of matches) {
        if (entry.child === null) {
          groups.set(entry.command, { parent: entry, children: [] });
          continue;
        }
        let group = groups.get(entry.command);
        if (!group) {
          commands ??= JSON.parse(inspector.commandEntries()) as Entry[];
          group = { parent: commands[entry.command], children: [] };
          groups.set(entry.command, group);
        }
        group.children.push(entry);
      }
    }
    const order = sort?.column === "id" ? undefined : sort;
    const parents = sortRows(
      [...groups.values()],
      order,
      (group, column) => group.parent[column],
    );
    for (const { parent, children } of parents) {
      const full = isFullPacket(parent);
      rows.push({ ...parent, hasChildren: children.length > 0 });
      if (
        (full ? fullPacketsOpen : packetsOpen) ||
        parent.command === revealedPacket
      ) {
        for (const child of sortRows(
          children,
          order,
          (entry, column) => entry[column],
        ))
          rows.push({
            ...child,
            hasChildren: false,
            ...(full ? { inFullPacket: true } : undefined),
          });
      }
    }
  }
  return {
    revision: ++revision,
    rows: rows.length,
    matches: matches.length,
    activity: points,
  };
}

const api: MessageIndexApi = {
  open(file) {
    opening = (async () => {
      const [, bytes] = await Promise.all([
        initInspector(),
        file.arrayBuffer(),
      ]);
      inspector = MessageInspector.fromBytes(new Uint8Array(bytes));
    })();
    return opening;
  },
  async query(input) {
    await opening;
    return query(input);
  },
  async range({ revision: request, start, end }) {
    await opening;
    return request === revision ? rows.slice(start, end) : [];
  },
  async position({ revision: request, command, child }) {
    await opening;
    return request === revision
      ? rows.findIndex((row) => row.command === command && row.child === child)
      : -1;
  },
  async occurrence({ revision: request, tick }) {
    await opening;
    return request === revision ? firstAtTick.get(tick) : undefined;
  },
  async entityChanges({ command, child }) {
    await opening;
    return JSON.parse(
      inspector.entityChanges(command, child),
    ) as MessageIndexCalls["entityChanges"]["output"];
  },
  async stringTables({ command, child, name }) {
    await opening;
    if (child === null) {
      return packetTableRecords(
        snapshotTables(name, JSON.parse(inspector.inspect(command))),
      );
    }
    const table = JSON.parse(
      inspector.stringTableChangeInfo(command, child),
    ) as [number, string] | null;
    return table ? [{ index: 0, id: table[0], name: table[1] }] : [];
  },
  async gameEvents({ command, child, id, tick }) {
    await opening;
    if (id === 205 && child !== null) {
      inspector.entries(tick);
      return gameEventDefinitions(
        JSON.parse(inspector.inspect(command, child)),
      );
    }
    const event = firedEvents().get(command + "/" + child);
    return event ? [event] : [];
  },
  async gameEventOccurrences({ id, name }) {
    await opening;
    const locations = [];
    for (const event of firedEvents().values()) {
      if (event.id === id && event.name === name)
        locations.push(event.tick, event.command, event.child);
    }
    const result = Uint32Array.from(locations);
    return transfer(result, [result.buffer]);
  },
  async typeOccurrences({ id, child }) {
    await opening;
    const entries = JSON.parse(inspector.search(String(id))) as Entry[];
    const locations = [];
    for (const entry of entries) {
      if (entry.id === id && (entry.child !== null) === child)
        locations.push(entry.tick, entry.command, entry.child ?? -1);
    }
    const result = Uint32Array.from(locations);
    return transfer(result, [result.buffer]);
  },
};
expose(api);
