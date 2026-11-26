import { SheetTable } from "../../workbench/SheetTable";
import { sortRows, type useTableSort } from "../../workbench/tableSort";
import { VirtualRows } from "../../workbench/VirtualRows";
import { useRef } from "react";

export interface EventField {
  name: string;
  kind: string;
  value?: string;
  index?: number;
}

export function EventFieldTable({
  rows = [],
  query,
  definition = false,
  error,
  sortControl: { sort, toggleSort },
}: {
  rows?: EventField[];
  query: string;
  definition?: boolean;
  error?: string;
  sortControl: ReturnType<typeof useTableSort>;
}) {
  const viewport = useRef<HTMLDivElement>(null);
  const term = query.trim().toLowerCase();
  const filtered = sortRows(
    rows.filter((row) =>
      [row.index?.toString(), row.name, row.kind, row.value].some((value) =>
        value?.toLowerCase().includes(term),
      ),
    ),
    sort,
    (row, column) => {
      if (column === "Index") return row.index;
      if (column === "Key") return row.name;
      if (column === "Value") return row.value;
      return row.kind;
    },
  );
  return (
    <SheetTable
      tableId={definition ? "event-definitions" : "event-values"}
      columns={
        definition ? ["Index", "Key", "Wire type"] : ["Key", "Type", "Value"]
      }
      compactIndex={definition}
      className={definition ? "event-columns" : undefined}
      viewportRef={viewport}
      sort={sort}
      onSort={toggleSort}
    >
      {error && (
        <p className="notice" role="alert">
          {error}
        </p>
      )}
      <VirtualRows
        viewport={viewport}
        count={filtered.length}
        renderRow={(index) => {
          const row = filtered[index];
          return definition ? (
            <div
              className="sheet-grid sheet-row"
              data-stripe={index % 2 === 1 || undefined}
            >
              <span>{row.index}</span>
              <span>{row.name}</span>
              <span className="field-type">{row.kind}</span>
            </div>
          ) : (
            <div className="sheet-grid message-field">
              <span>{row.name}</span>
              <span className="field-type">{row.kind}</span>
              <span className="field-value">{row.value}</span>
            </div>
          );
        }}
      />
    </SheetTable>
  );
}
