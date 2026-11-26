import type { ReplaySession } from "../generated/wasm/inspector";
import { readFields } from "./readFields";
import { readStringTable } from "./readStringTable";
import type { Comparison, InspectionView } from "./state";

export function seekReplay(session: ReplaySession, tick: number) {
  session.runToTick(tick);
  return session.tick();
}

export function captureComparison(
  session: ReplaySession,
  tick: number,
  view: InspectionView,
  entity: number | undefined,
  table: string | undefined,
): Comparison | undefined {
  if (view === "stringTables" && table !== undefined) {
    return { tick, view, table, rows: readStringTable(session, table) };
  }
  if (
    (view === "entities" || view === "baselineEntities") &&
    entity !== undefined
  ) {
    const fields = readFields(session, view, entity);
    if (fields) return { tick, view, index: entity, fields };
  }
}
