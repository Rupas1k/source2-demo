import type { EntityFieldRecord } from "../../generated/wasm/inspector";

export function compareFieldPaths(
  a: Pick<EntityFieldRecord, "path">,
  b: Pick<EntityFieldRecord, "path">,
) {
  for (let i = 0; i < Math.min(a.path.length, b.path.length); i++) {
    if (a.path[i] !== b.path[i]) {
      return a.path[i] - b.path[i];
    }
  }

  return a.path.length - b.path.length;
}
