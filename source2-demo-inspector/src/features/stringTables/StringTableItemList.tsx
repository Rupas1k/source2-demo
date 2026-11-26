import { StringTableRows } from "./StringTableRows";
import { compareTableRows } from "./stringTableChanges";
import { readStringTable } from "../../replay/readStringTable";
import { FilterToggle, SearchBar } from "../../workbench/PaneControls";
import { useMemo, useRef, useState } from "react";
import { useReplayStore } from "../../replay/state";

function formatData(data?: Uint8Array | null) {
  return data == null ? null : `${data.length} bytes`;
}

export function StringTableItemList({
  tableName,
  selectedRow,
  onSelectRow,
}: {
  tableName: string | undefined;
  selectedRow: number | undefined;
  onSelectRow: (index: number) => void;
}) {
  const session = useReplayStore((state) => state.replaySession);
  const [query, setQuery] = useState("");
  const tick = useReplayStore((state) => state.currentTick);

  const comparison = useReplayStore((state) => state.comparison);
  const previous =
    comparison?.view === "stringTables" && comparison.table === tableName
      ? comparison
      : undefined;
  const [changes, setChanges] = useState(false);
  const entries = useMemo(() => {
    void tick;
    return session && tableName !== undefined
      ? readStringTable(session, tableName)
      : [];
  }, [session, tableName, tick]);
  const rows = useMemo(
    () =>
      compareTableRows(entries, changes ? previous?.rows : undefined).map(
        (entry) => ({
          ...entry,
          keyText: (entry.current ?? entry.previous)?.key,
          dataSize: (entry.current ?? entry.previous)?.data?.length,
          oldData: entry.dataChanged ? formatData(entry.previous?.data) : null,
          data: formatData(entry.current?.data),
          searchValues: [
            entry.current?.key,
            entry.previous?.key,
            formatData(entry.current?.data),
            entry.dataChanged ? formatData(entry.previous?.data) : null,
          ],
        }),
      ),
    [entries, changes, previous],
  );
  const viewportRef = useRef<HTMLDivElement>(null);

  return (
    <div className="sheet">
      <SearchBar label="Filter entries" value={query} onChange={setQuery}>
        {previous && (
          <FilterToggle
            pressed={changes}
            title={`Compare with tick ${previous.tick}`}
            onClick={() => {
              setChanges(!changes);
              viewportRef.current?.scrollTo({ top: 0 });
            }}
          >
            Changes
          </FilterToggle>
        )}
      </SearchBar>
      <StringTableRows
        rows={rows}
        renderCells={(entry) => ({
          keyCell: renderTableCell({
            changed: entry.keyChanged,
            before: entry.previous?.key,
            after: entry.current?.key,
            added: !entry.previous,
            removed: !entry.current,
          }),
          dataCell: renderTableCell({
            changed: entry.dataChanged,
            before: entry.oldData,
            after: entry.data,
            added: !entry.previous,
            removed: !entry.current,
          }),
        })}
        query={query}
        viewport={viewportRef}
        selectedRow={selectedRow}
        onSelectRow={onSelectRow}
      />
    </div>
  );
}

function renderTableCell({
  changed,
  before,
  after,
  added,
  removed,
}: {
  changed: boolean;
  before?: string | null;
  after?: string | null;
  added: boolean;
  removed: boolean;
}) {
  if (!changed) return after;
  const prior = added ? "absent" : (before ?? "—");
  const current = removed ? "absent" : (after ?? "—");
  const title = `${prior} → ${current}`;
  return (
    <>
      <span className="prior-value" title={title}>
        {prior} →{" "}
      </span>
      <span className="table-change-value" title={title}>
        {current}
      </span>
    </>
  );
}
