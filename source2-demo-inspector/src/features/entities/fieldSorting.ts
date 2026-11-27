import type { EntityFieldRow } from "./entityTypes";

export const fieldColumns = [
  "Field",
  "Encoded type",
  "Decoded type",
  "Value",
] as const;
export type Column = (typeof fieldColumns)[number];
export type FieldSort = {
  column: Column;
  direction: "ascending" | "descending";
};
const collator = new Intl.Collator(undefined, {
  numeric: true,
  sensitivity: "base",
});
const decimal = /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:e[+-]?\d+)?$/i;

function cell(row: EntityFieldRow, column: Column) {
  switch (column) {
    case "Field":
      return row.joinedNamedPath;
    case "Encoded type":
      return row.inner.encodedAs;
    case "Decoded type":
      return row.inner.decodedAs === "None" ? "" : row.inner.decodedAs;
    case "Value":
      return String(row.collectionLength ?? row.inner.value);
  }
}

function compare(a: string, b: string, column: Column) {
  if (column === "Value") {
    const an = decimal.test(a),
      bn = decimal.test(b);
    if (an !== bn) return an ? -1 : 1;
    if (an) {
      const av = /^[+-]?\d+$/.test(a) ? BigInt(a) : Number(a);
      const bv = /^[+-]?\d+$/.test(b) ? BigInt(b) : Number(b);
      return av < bv ? -1 : av > bv ? 1 : 0;
    }
  }
  return collator.compare(a, b);
}

export function sortFields(rows: EntityFieldRow[], sort?: FieldSort) {
  if (!sort) return rows;
  const groups = new Set(
    rows.filter((row) => row.expandableKind).map((row) => row.joinedNamedPath),
  );
  const children = new Map<string, EntityFieldRow[]>();
  for (const row of rows) {
    let path = row.joinedNamedPath;
    let parent = "";
    while (path.includes(".")) {
      path = path.slice(0, path.lastIndexOf("."));
      if (groups.has(path)) {
        parent = path;
        break;
      }
    }
    const siblings = children.get(parent);
    if (siblings) siblings.push(row);
    else children.set(parent, [row]);
  }
  const result: EntityFieldRow[] = [];
  const direction = sort.direction === "ascending" ? 1 : -1;
  function visit(parent: string) {
    const siblings = children.get(parent);
    if (!siblings) return;
    children.delete(parent);
    siblings.sort(
      (a, b) =>
        direction *
        compare(cell(a, sort!.column), cell(b, sort!.column), sort!.column),
    );
    for (const row of siblings) {
      result.push(row);
      if (row.expandableKind) visit(row.joinedNamedPath);
    }
  }
  visit("");
  return result;
}
