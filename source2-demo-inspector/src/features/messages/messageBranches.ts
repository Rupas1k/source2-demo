import type { NestedMessageBranchKind } from "../../app/preferences";
import type { Entry } from "./messageIndex";

export function branchKind(entry: Entry): NestedMessageBranchKind | undefined {
  if (entry.child === null) {
    if (entry.name === "CDemoFullPacket" || entry.name === "CDemoStringTables")
      return "stringTables";
  } else {
    if (entry.id === 55) return "entities";
    if (entry.id === 44 || entry.id === 45) return "stringTables";
    if (entry.id === 205 || entry.id === 207) return "gameEvents";
  }
}
