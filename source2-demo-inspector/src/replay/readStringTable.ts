import type { ReplaySession } from "../generated/wasm/inspector";

export interface StringTableRow {
  index: number;
  key: string | null;
  data: Uint8Array | null;
}

const decoder = new TextDecoder();

export function readStringTable(
  parser: ReplaySession,
  name: string,
): StringTableRow[] {
  return (parser.listStringTableItems(name) ?? []).map((entry) => {
    const { index, string, userData } = entry;
    entry.free();
    return {
      index,
      key: string ? decoder.decode(string) : null,
      data: userData ?? null,
    };
  });
}
