import { useState } from "react";

export interface TableSort<Column extends string = string> {
  column: Column;
  direction: "ascending" | "descending";
}

export function useTableSort<Column extends string = string>(
  initial?: TableSort<Column>,
) {
  const [sort, setSort] = useState(initial);
  function toggleSort(column: Column) {
    setSort((previous) => {
      if (previous?.column !== column)
        return { column, direction: "ascending" };
      return previous.direction === "ascending"
        ? { column, direction: "descending" }
        : undefined;
    });
  }
  return { sort, toggleSort };
}

const collator = new Intl.Collator(undefined, {
  numeric: true,
  sensitivity: "base",
});

export function sortRows<Row, Column extends string>(
  rows: Row[],
  sort: TableSort<Column> | undefined,
  value: (row: Row, column: Column) => string | number | null | undefined,
): Row[] {
  if (!sort) return rows;
  const direction = sort.direction === "ascending" ? 1 : -1;
  return [...rows].sort((a, b) => {
    const left = value(a, sort.column);
    const right = value(b, sort.column);
    if (left == null || right == null)
      return Number(left == null) - Number(right == null);
    const order =
      typeof left === "number" && typeof right === "number"
        ? left - right
        : collator.compare(String(left), String(right));
    return direction * order;
  });
}
