import type { ReactNode, RefObject } from "react";
import { SheetTable } from "../../workbench/SheetTable";
import { sortRows, useTableSort } from "../../workbench/tableSort";
import { VirtualRows } from "../../workbench/VirtualRows";

interface EntryRow {
  index: number;
  keyText?: string | null;
  dataSize?: number;
  searchValues?: (string | null | undefined)[];
}

export function StringTableRows<Row extends EntryRow>({
  rows,
  renderCells,
  query,
  viewport,
  selectedRow,
  onSelectRow,
  error,
  limit,
  showMore,
}: {
  rows: Row[];
  renderCells: (row: Row) => { keyCell: ReactNode; dataCell: ReactNode };
  query: string;
  viewport?: RefObject<HTMLDivElement | null>;
  selectedRow?: number;
  onSelectRow?: (index: number) => void;
  error?: string;
  limit?: number;
  showMore?: () => void;
}) {
  const { sort, toggleSort } = useTableSort();
  const term = query.trim().toLowerCase();
  const ordered = sortRows(
    term
      ? rows.filter(
          (row) =>
            (row.searchValues
              ? String(row.index).includes(term)
              : String(row.index) === term) ||
            (row.searchValues ?? [row.keyText]).some((value) =>
              value?.toLowerCase().includes(term),
            ),
        )
      : rows,
    sort,
    (row, column) =>
      column === "Row"
        ? row.index
        : column === "Key"
          ? row.keyText
          : row.dataSize,
  );

  function renderRow(position: number) {
    const row = ordered[position];
    const { keyCell, dataCell } = renderCells(row);
    return (
      <div
        key={row.index}
        className={
          onSelectRow ? "sheet-grid sheet-row" : "sheet-grid message-field"
        }
        data-stripe={(onSelectRow && position % 2 === 1) || undefined}
        data-selected={selectedRow === row.index || undefined}
        onClick={onSelectRow && (() => onSelectRow(row.index))}
      >
        <span>
          {onSelectRow ? (
            <button
              type="button"
              title="Show timeline activity for this row"
              aria-label={`Show activity for row ${row.index}`}
              aria-pressed={selectedRow === row.index}
              onClick={(event) => {
                event.stopPropagation();
                onSelectRow(row.index);
              }}
            >
              {row.index}
            </button>
          ) : (
            row.index
          )}
        </span>
        <span>{keyCell}</span>
        <span className={onSelectRow ? "field-type" : undefined}>
          {dataCell}
        </span>
      </div>
    );
  }

  return (
    <SheetTable
      tableId={viewport ? "string-tables" : "packet-string-tables"}
      columns={["Row", "Key", "Data"]}
      sort={sort}
      onSort={toggleSort}
      viewportRef={viewport}
      className="entry-columns"
      compactIndex
    >
      {error && (
        <p className="notice" role="alert">
          {error}
        </p>
      )}
      {viewport ? (
        <VirtualRows
          viewport={viewport}
          count={ordered.length}
          renderRow={renderRow}
        />
      ) : (
        ordered.slice(0, limit).map((_, index) => renderRow(index))
      )}
      {limit !== undefined && ordered.length > limit && (
        <button className="record-row" onClick={showMore}>
          Show more entries
        </button>
      )}
    </SheetTable>
  );
}
