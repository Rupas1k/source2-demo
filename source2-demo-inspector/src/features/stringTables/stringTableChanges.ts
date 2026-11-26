import type { StringTableRow } from "../../replay/readStringTable";

export interface TableRowComparison {
  index: number;
  previous?: StringTableRow;
  current?: StringTableRow;
  keyChanged: boolean;
  dataChanged: boolean;
}

function sameBytes(a: Uint8Array | null, b: Uint8Array | null) {
  return (
    a === b ||
    (a !== null &&
      b !== null &&
      a.length === b.length &&
      a.every((byte, i) => byte === b[i]))
  );
}

export function compareTableRows(
  current: StringTableRow[],
  previous?: StringTableRow[],
): TableRowComparison[] {
  if (!previous)
    return current.map((row) => ({
      index: row.index,
      current: row,
      keyChanged: false,
      dataChanged: false,
    }));
  const oldRows = new Map(previous.map((row) => [row.index, row]));
  const changes = current.flatMap((row) => {
    const old = oldRows.get(row.index);
    oldRows.delete(row.index);
    const keyChanged = !old || row.key !== old.key;
    const dataChanged = !old || !sameBytes(row.data, old.data);
    return keyChanged || dataChanged
      ? [
          {
            index: row.index,
            previous: old,
            current: row,
            keyChanged,
            dataChanged,
          },
        ]
      : [];
  });
  return [
    ...changes,
    ...Array.from(oldRows.values(), (row) => ({
      index: row.index,
      previous: row,
      keyChanged: true,
      dataChanged: true,
    })),
  ];
}
