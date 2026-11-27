import { readReplayInput } from "../../replay/readReplayInput";
import { expose, transfer } from "comlink";
import { gameEventDefinitions } from "./packetGameEvents";
import type { FiredEvent } from "../../replay/readGameEvents";
import { packetTableRecords, snapshotTables } from "./packetStringTables";
import initInspector, {
  type MessageInspector,
} from "../../generated/wasm/inspector";
import { sortRows } from "../../workbench/tableSort";
import type {
  Entry,
  IndexedEntry,
  MessageIndexApi,
  MessageIndexCalls,
  Query,
} from "./messageIndex";

// Search results use four words per match: command, child, wire ID, and tick.
const matchWidth = 4;
const noChild = 0xffffffff;
const parentRow = 0x80000000;

interface PacketGroup {
  parent: Entry;
  childStart: number;
  childEnd: number;
}

let inspector: MessageInspector;
let opening: Promise<void>;
let commands: Entry[] | undefined;
let lastTerm: string | undefined;
let matches: Uint32Array = new Uint32Array();
let rows = new Uint32Array();
let groups: Map<number, PacketGroup> | undefined;
let parents: PacketGroup[] = [];
let firstAtTick = new Map<number, number>();
let points = new Uint32Array();
let revision = 0;
let fired: Map<string, FiredEvent> | undefined;
const names = {
  command: new Map<number, string>(),
  packet: new Map<number, string>(),
};

function messageName(packet: boolean, id: number) {
  const cache = packet ? names.packet : names.command;
  let name = cache.get(id);
  if (name === undefined) {
    name = inspector.messageName(packet, id);
    cache.set(id, name);
  }
  return name;
}

function matchEntry(offset: number): Entry {
  const child = matches[offset + 1];
  const id = matches[offset + 2];
  return {
    command: matches[offset],
    child: child === noChild ? null : child,
    id,
    tick: matches[offset + 3],
    name: messageName(child !== noChild, id),
  };
}

function firedEvents() {
  return (fired ??= new Map(
    (JSON.parse(inspector.firedEvents()) as FiredEvent[]).map((event) => [
      event.command + "/" + event.child,
      event,
    ]),
  ));
}

function indexMatches(term: string) {
  matches = inspector.search(term);
  const counts = new Map<number, number>();
  firstAtTick = new Map();
  for (let offset = 0; offset < matches.length; offset += matchWidth) {
    const tick = matches[offset + 3];
    if (!firstAtTick.has(tick)) firstAtTick.set(tick, offset);
    if (term) counts.set(tick, (counts.get(tick) ?? 0) + 1);
  }
  points = Uint32Array.from(Array.from(counts).flat());
  lastTerm = term;
  groups = undefined;
}

function groupMatches() {
  const result = new Map<number, PacketGroup>();
  for (let offset = 0; offset < matches.length; offset += matchWidth) {
    const command = matches[offset];
    const isParent = matches[offset + 1] === noChild;
    let group = result.get(command);
    if (!group) {
      if (!isParent)
        commands ??= JSON.parse(inspector.commandEntries()) as Entry[];
      const start = isParent ? offset + matchWidth : offset;
      group = {
        parent: isParent ? matchEntry(offset) : commands![command],
        childStart: start,
        childEnd: start,
      };
      result.set(command, group);
    }
    if (!isParent) group.childEnd = offset + matchWidth;
  }
  return result;
}

function query({ term, wide, sort, packetExpansion, revealedPacket }: Query) {
  if (lastTerm !== term) indexMatches(term);
  rows = new Uint32Array();
  parents = [];
  if (wide) {
    groups ??= groupMatches();
    const order = sort?.column === "id" ? undefined : sort;
    parents = sortRows(
      [...groups.values()],
      order,
      (group, column) => group.parent[column],
    );
    const isOpen = (group: PacketGroup) =>
      group.parent.command === revealedPacket ||
      (packetExpansion.get(group.parent.command) ??
        revealsChildren(group, term));
    const count = parents.reduce(
      (total, group) =>
        total +
        (isOpen(group) ? (group.childEnd - group.childStart) / matchWidth : 0),
      parents.length,
    );
    rows = new Uint32Array(count);
    let position = 0;
    for (const [index, group] of parents.entries()) {
      rows[position++] = parentRow + index;
      if (!isOpen(group)) continue;
      const { childStart, childEnd } = group;
      if (order?.column === "name") {
        const offsets = [];
        for (let offset = childStart; offset < childEnd; offset += matchWidth)
          offsets.push(offset);
        const sorted = sortRows(offsets, order, (offset) =>
          messageName(true, matches[offset + 2]),
        );
        rows.set(sorted, position);
        position += sorted.length;
      } else {
        for (let offset = childStart; offset < childEnd; offset += matchWidth)
          rows[position++] = offset;
      }
    }
  } else {
    groups = undefined;
  }
  return {
    revision: ++revision,
    rows: rows.length,
    matches: matches.length / matchWidth,
    activity: points,
  };
}

function revealsChildren(group: PacketGroup, term: string | undefined) {
  return !!term && group.childStart < group.childEnd;
}

function rowEntry(row: number): IndexedEntry {
  if (row >= parentRow) {
    const group = parents[row - parentRow];
    return {
      ...group.parent,
      hasChildren: group.childStart < group.childEnd,
      ...(revealsChildren(group, lastTerm)
        ? { revealChildren: true }
        : undefined),
    };
  }
  return { ...matchEntry(row), hasChildren: false };
}

const api: MessageIndexApi = {
  open(file) {
    opening = (async () => {
      await initInspector();
      inspector = (await readReplayInput(file)).intoMessages();
    })();
    return opening;
  },
  async query(input) {
    await opening;
    return query(input);
  },
  async range({ revision: request, start, end }) {
    await opening;
    return request === revision
      ? Array.from(rows.subarray(start, end), rowEntry)
      : [];
  },
  async position({ revision: request, command, child }) {
    await opening;
    return request === revision
      ? rows.findIndex((row) =>
          row >= parentRow
            ? child === null &&
              parents[row - parentRow].parent.command === command
            : child !== null &&
              matches[row] === command &&
              matches[row + 1] === child,
        )
      : -1;
  },
  async occurrence({ revision: request, tick }) {
    await opening;
    const offset = firstAtTick.get(tick);
    return request === revision && offset !== undefined
      ? matchEntry(offset)
      : undefined;
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
    const entries = inspector.search(String(id));
    const locations = [];
    for (let offset = 0; offset < entries.length; offset += matchWidth) {
      const entryChild = entries[offset + 1];
      if (entries[offset + 2] === id && (entryChild !== noChild) === child)
        locations.push(entries[offset + 3], entries[offset], entryChild);
    }
    const result = Uint32Array.from(locations);
    return transfer(result, [result.buffer]);
  },
};
expose(api);
