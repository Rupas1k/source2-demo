import type { ReplaySession } from "../generated/wasm/inspector";
import { readFields } from "./readFields";
import { readStringTable } from "./readStringTable";
import type { Comparison, InspectionView } from "./state";

export function seekReplay(session: ReplaySession, tick: number) {
  session.runToTick(tick);
  return session.tick();
}

export function adjacentRecordedTick(
  ticks: Int32Array,
  tick: number,
  forward: boolean,
) {
  let lo = 0;
  let hi = ticks.length;
  while (lo < hi) {
    const mid = lo + Math.floor((hi - lo) / 2);
    if (ticks[mid] < tick || (forward && ticks[mid] === tick)) lo = mid + 1;
    else hi = mid;
  }
  return ticks[forward ? lo : lo - 1] as number | undefined;
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
