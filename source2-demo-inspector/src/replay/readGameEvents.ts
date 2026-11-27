import type { ReplaySession } from "../generated/wasm/inspector";

export interface FiredEvent {
  id: number;
  name: string;
  tick: number;
  command: number;
  child: number;
  fields: { name: string; kind: string; value: string }[];
}

export function readFiredEvents(session: ReplaySession): FiredEvent[] {
  const inspector = session.openMessages();
  try {
    return JSON.parse(inspector.firedEvents());
  } finally {
    inspector.free();
  }
}

export function readEventDefinitions(session: ReplaySession | undefined) {
  return (session?.listGameEvents() ?? []).map((entry) => {
    const { id, name } = entry;
    entry.free();
    return { key: id, id, name };
  });
}

export function readEventKeys(
  session: ReplaySession | undefined,
  id: number | undefined,
) {
  if (id === undefined) return;
  return session?.listGameEventKeys(id)?.map((record) => {
    const { index, name, typeName, typeId } = record;
    record.free();
    return {
      index,
      name,
      kind: typeName === "unknown" ? `unknown (${typeId})` : typeName,
    };
  });
}
