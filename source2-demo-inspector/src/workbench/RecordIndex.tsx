import { nextRecordIndex } from "./recordNavigation";
import { VirtualRecordList } from "./VirtualRecordList";
import { SortLabel } from "./FieldColumnHeader";
import { sortRows, useTableSort, type TableSort } from "./tableSort";
import { SearchBar } from "./PaneControls";
import { IndexHeading } from "./SheetTable";
import type { VirtualizerHandle } from "virtua";
import {
  type ReactNode,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
} from "react";

export interface IndexRecord<K extends string | number> {
  key: K;
  name: string;
  id: number;
  status?: string;
  tick?: number;
}

type IndexColumn = "name" | "tick" | "id";

interface Props<K extends string | number> {
  records: IndexRecord<K>[];
  selected: K | undefined;
  onSelect: (key: K | undefined) => void;
  onActivate?: (key: K) => void;
  onQueryChange?: (query: string) => void;
  filterLabel: string;
  heading: string;
  searchIds?: boolean;
  tickColumn?: boolean;
  filterAction?: ReactNode;
  sortControl?: {
    sort?: TableSort<IndexColumn>;
    toggleSort: (column: IndexColumn) => void;
  };
  reveal?: { index: number; key?: K };
}

export function RecordIndex<K extends string | number>({
  records,
  selected,
  onSelect,
  onActivate,
  onQueryChange,
  filterLabel,
  heading,
  searchIds,
  tickColumn,
  filterAction,
  sortControl,
  reveal,
}: Props<K>) {
  const localSort = useTableSort<IndexColumn>();
  const { sort, toggleSort } = sortControl ?? localSort;
  const [query, setQuery] = useState("");
  const term = query.trim().toLowerCase();
  const rows = useMemo(() => {
    const filtered = records.filter(
      (row) =>
        row.name.toLowerCase().includes(term) ||
        (searchIds && String(row.id).includes(term)) ||
        (tickColumn && String(row.tick) === term),
    );
    return sortControl
      ? filtered
      : sortRows(filtered, sort, (row, column) => row[column]);
  }, [records, searchIds, sort, sortControl, term, tickColumn]);
  const viewport = useRef<HTMLDivElement>(null);
  const list = useRef<VirtualizerHandle>(null);
  const handled = useRef<typeof reveal>(undefined);
  useLayoutEffect(() => {
    if (!reveal || reveal === handled.current) return;
    const matches = (row: IndexRecord<K>) =>
      reveal.key === undefined
        ? row.id === reveal.index
        : row.key === reveal.key;
    const position = rows.findIndex(matches);
    if (position >= 0)
      list.current?.scrollToIndex(position, { align: "nearest" });
    else if (query && records.some(matches)) {
      setQuery("");
      return;
    }
    handled.current = reveal;
  }, [rows, records, list, query, reveal]);

  function navigate(event: React.KeyboardEvent) {
    if (event.defaultPrevented || event.altKey || !rows.length) return;
    if (event.key === "Enter" && event.target === event.currentTarget) {
      event.preventDefault();
      const key = rows.find((row) => row.key === selected)?.key ?? rows[0].key;
      (onActivate ?? onSelect)(key);
      return;
    }
    if (event.key !== "ArrowUp" && event.key !== "ArrowDown") return;
    event.preventDefault();
    const current = rows.findIndex((row) => row.key === selected);
    const next = nextRecordIndex(
      rows.length,
      current,
      event.key === "ArrowDown",
    );
    onSelect(rows[next].key);
    list.current?.scrollToIndex(next, { align: "nearest" });
  }
  const columnLabel = (column: IndexColumn, label: string) => (
    <SortLabel
      label={label}
      direction={sort?.column === column ? sort.direction : undefined}
      onSort={() => {
        toggleSort(column);
        viewport.current?.scrollTo({ top: 0 });
      }}
    />
  );
  return (
    <div className="sheet outline-none" tabIndex={0} onKeyDown={navigate}>
      <SearchBar
        label={filterLabel}
        value={query}
        onChange={(value) => {
          setQuery(value);
          onQueryChange?.(value);
        }}
        matchCount={term ? rows.length : undefined}
      >
        {filterAction}
      </SearchBar>
      <IndexHeading
        label={columnLabel("name", heading)}
        value={
          tickColumn ? columnLabel("tick", "Tick") : columnLabel("id", "ID")
        }
      />
      <VirtualRecordList
        viewport={viewport}
        list={list}
        count={rows.length}
        renderRow={(index) => {
          const row = rows[index];
          if (!row) return null;
          return (
            <div
              key={row.key}
              className={
                "record-row record-row__selectable flex items-center" +
                (selected === row.key ? " record-row__selected" : "")
              }
              style={{ height: 26 }}
              onClick={() =>
                onSelect(row.key === selected ? undefined : row.key)
              }
            >
              <span className="truncate" title={row.name}>
                {row.name}
              </span>
              {row.status && (
                <span className="record-status">{row.status}</span>
              )}
              <span className="entity-id">
                {tickColumn ? row.tick?.toLocaleString() : row.id}
              </span>
            </div>
          );
        }}
      />
    </div>
  );
}
