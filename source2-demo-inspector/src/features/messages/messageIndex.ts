import type { PacketGameEvent } from "./packetGameEvents";
import type { FiredEvent } from "../../replay/readGameEvents";
import type { PacketTable } from "./packetStringTables";
import type { EntityChange } from "./EntityChangeFields";
import type { TableSort } from "../../workbench/tableSort";

export interface Entry {
  command: number;
  child: number | null;
  id: number;
  name: string;
  tick: number;
}

export function isFullPacket(entry: Entry) {
  return entry.child === null && entry.name === "CDemoFullPacket";
}

export interface IndexedEntry extends Entry {
  hasChildren: boolean;
  inFullPacket?: true;
}

export interface Query {
  term: string;
  wide: boolean;
  sort?: TableSort<"name" | "id" | "tick">;
  packetsOpen: boolean;
  fullPacketsOpen: boolean;
  revealedPacket?: number;
}

export interface MessageIndexCalls {
  open: { input: File; output: void };
  query: {
    input: Query;
    output: {
      revision: number;
      rows: number;
      matches: number;
      activity: Uint32Array;
    };
  };
  range: {
    input: { revision: number; start: number; end: number };
    output: IndexedEntry[];
  };
  position: {
    input: Pick<Entry, "command" | "child"> & { revision: number };
    output: number;
  };
  occurrence: {
    input: { revision: number; tick: number };
    output: Entry | undefined;
  };
  entityChanges: {
    input: { command: number; child: number };
    output: EntityChange[];
  };
  stringTables: {
    input: Pick<Entry, "command" | "child" | "name">;
    output: PacketTable[];
  };
  gameEvents: {
    input: Pick<Entry, "command" | "child" | "id" | "tick">;
    output: PacketGameEvent[];
  };
  gameEventOccurrences: {
    input: Pick<FiredEvent, "id" | "name">;
    output: Uint32Array;
  };
  typeOccurrences: {
    input: { id: number; child: boolean };
    output: Uint32Array;
  };
}

export type IndexMethod = keyof MessageIndexCalls;
export type MessageIndexApi = {
  [K in IndexMethod]: (
    input: MessageIndexCalls[K]["input"],
  ) => MessageIndexCalls[K]["output"] | Promise<MessageIndexCalls[K]["output"]>;
};

export function activityPoints(data: Uint32Array) {
  const points = [];
  for (let i = 0; i < data.length; i += 2)
    points.push({ tick: data[i], count: data[i + 1] });
  return points;
}
