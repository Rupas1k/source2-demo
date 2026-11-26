import type { VirtualizerHandle } from "virtua";
import { SearchBar } from "../../workbench/PaneControls";
import { SheetTable } from "../../workbench/SheetTable";
import { fieldColumns, sortFields, type FieldSort } from "./fieldSorting";
import { FieldCells } from "./FieldCells";
import { VirtualRows } from "../../workbench/VirtualRows";
import {
  useCallback,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";
import type { FieldSnapshot } from "../../replay/readFields";
import {
  buildEntityFieldRows,
  filterEntityFieldRows,
} from "./entityFieldGrouping";

export function useEntityFieldTable(
  rawEntityFieldList: FieldSnapshot[] | undefined,
  previous?: { fields: FieldSnapshot[] },
  initialPath?: string,
) {
  const [expandedFieldGroups, setExpandedFieldGroups] = useState(
    () =>
      new Set(
        initialPath
          ? rawEntityFieldList
              ?.find((field) => field.path.join(".") === initialPath)
              ?.namedPath.map((_, index, parts) =>
                parts.slice(0, index).join("."),
              )
          : [],
      ),
  );
  const entityFieldList = useMemo(
    () => buildEntityFieldRows(rawEntityFieldList, expandedFieldGroups),
    [rawEntityFieldList, expandedFieldGroups],
  );
  const searchableEntityFieldList = useMemo(
    () => buildEntityFieldRows(rawEntityFieldList, "all"),
    [rawEntityFieldList],
  );

  const [query, setQuery] = useState("");
  const [changes, setChanges] = useState(false);
  const previousRows = useMemo(
    () => previous && buildEntityFieldRows(previous.fields, "all"),
    [previous],
  );
  const prior = useMemo(
    () => new Map(previousRows?.map((row) => [row.joinedNamedPath, row])),
    [previousRows],
  );
  const comparing = !!previous && changes;
  const typeWidths = useMemo(() => {
    const context = document.createElement("canvas").getContext("2d");
    const baseWidth = 124;
    if (!context) return { encoded: baseWidth, decoded: baseWidth };
    context.font = "13px system-ui";
    const rows =
      comparing && previous
        ? searchableEntityFieldList.concat(previousRows ?? [])
        : searchableEntityFieldList;
    function measure(key: "encodedAs" | "decodedAs") {
      let width = baseWidth;
      for (const type of new Set(rows.map((row) => row.inner[key]))) {
        width = Math.max(
          width,
          Math.ceil(context!.measureText(type).width) + 22,
        );
      }
      return width;
    }
    return { encoded: measure("encodedAs"), decoded: measure("decodedAs") };
  }, [searchableEntityFieldList, comparing, previous]);
  const current = useMemo(
    () =>
      new Map(
        searchableEntityFieldList.map((row) => [row.joinedNamedPath, row]),
      ),
    [searchableEntityFieldList],
  );
  const rows = comparing
    ? [
        ...searchableEntityFieldList,
        ...(previousRows ?? []).filter(
          (row) => !current.has(row.joinedNamedPath),
        ),
      ].filter(
        (row) =>
          current.get(row.joinedNamedPath)?.inner.value !==
          prior.get(row.joinedNamedPath)?.inner.value,
      )
    : query
      ? searchableEntityFieldList
      : entityFieldList;
  const term = query.trim().toLowerCase();
  const [sort, setSort] = useState<FieldSort>();
  const matches = term
    ? filterEntityFieldRows(rows, (value) => value.toLowerCase().includes(term))
    : rows;
  const filteredEntityFieldList = useMemo(
    () => sortFields(matches, sort),
    [matches, sort],
  );
  const visibleFieldGroups = useMemo(() => {
    const groupNames = new Set<string>();
    for (const row of filteredEntityFieldList ?? []) {
      for (let i = 1; i < row.inner.namedPath.length; i++) {
        groupNames.add(row.inner.namedPath.slice(0, i).join("."));
      }
    }
    return groupNames;
  }, [filteredEntityFieldList]);

  const expandablePaths = useMemo(
    () =>
      searchableEntityFieldList
        .filter((row) => row.expandableKind)
        .map((row) => row.joinedNamedPath),
    [searchableEntityFieldList],
  );
  const allExpanded =
    expandablePaths.length > 0 &&
    expandablePaths.every((path) => expandedFieldGroups.has(path));
  const columnSizes = {
    Field: { minimum: 160, grow: 2 },
    "Encoded type": {
      minimum: 124,
      maximum: `min(${typeWidths.encoded}px, 27%)`,
    },
    "Decoded type": {
      minimum: 124,
      maximum: `min(${typeWidths.decoded}px, 27%)`,
    },
    Value: { minimum: 80, grow: 1 },
  };

  const viewportRef = useRef<HTMLDivElement>(null);
  const virtualizer = useRef<VirtualizerHandle>(null);

  const handleFieldGroupToggle = useCallback((groupName: string) => {
    setExpandedFieldGroups((prevExpandedGroups) => {
      const nextExpandedGroups = new Set(prevExpandedGroups);
      if (nextExpandedGroups.has(groupName)) {
        nextExpandedGroups.delete(groupName);
      } else {
        nextExpandedGroups.add(groupName);
      }
      return nextExpandedGroups;
    });
  }, []);

  const revealIndex =
    initialPath === undefined
      ? -1
      : filteredEntityFieldList.reduce(
          (selected, row, index) =>
            !row.expandableKind && row.inner.path.join(".") === initialPath
              ? index
              : selected,
          -1,
        );
  const revealed = useRef(false);
  useLayoutEffect(() => {
    if (revealIndex < 0 || revealed.current) return;
    revealed.current = true;
    virtualizer.current?.scrollToIndex(revealIndex, { align: "nearest" });
  }, [revealIndex, virtualizer]);

  return {
    query,
    setQuery,
    changes,
    setChanges,
    comparing,
    prior,
    current,
    columnSizes,
    expandedFieldGroups,
    setExpandedFieldGroups,
    allExpanded,
    expandablePaths,
    term,
    viewportRef,
    virtualizer,
    sort,
    setSort,
    filteredEntityFieldList,
    visibleFieldGroups,
    handleFieldGroupToggle,
  };
}

export function EntityFieldTable({
  table,
  actions,
  error,
  onFollow,
  onSource,
  partial = false,
}: {
  table: ReturnType<typeof useEntityFieldTable>;
  actions?: ReactNode;
  error?: string;
  onFollow: (index: number, handle: number) => void;
  onSource?: (path: Uint16Array, name: string) => void;
  partial?: boolean;
}) {
  const {
    query,
    setQuery,
    comparing,
    prior,
    current,
    columnSizes,
    setExpandedFieldGroups,
    expandedFieldGroups,
    allExpanded,
    expandablePaths,
    term,
    viewportRef,
    virtualizer,
    sort,
    setSort,
    filteredEntityFieldList,
    visibleFieldGroups,
    handleFieldGroupToggle,
  } = table;
  return (
    <div className="sheet">
      <SearchBar label="Filter fields" value={query} onChange={setQuery}>
        {expandablePaths.length > 0 && !term && !comparing && (
          <button
            type="button"
            onClick={() =>
              setExpandedFieldGroups(
                new Set(allExpanded ? [] : expandablePaths),
              )
            }
          >
            {allExpanded ? "Collapse all" : "Expand all"}
          </button>
        )}
        {actions}
      </SearchBar>
      <SheetTable
        tableId="entity-fields"
        columns={fieldColumns}
        className="entity-fields field-tree"
        sizes={fieldColumns.map((column) => columnSizes[column])}
        viewportRef={viewportRef}
        sort={sort}
        onSort={(column) => {
          setSort((active) =>
            active?.column !== column
              ? { column, direction: "ascending" }
              : active.direction === "ascending"
                ? { column, direction: "descending" }
                : undefined,
          );
          viewportRef.current?.scrollTo({ top: 0 });
        }}
      >
        {error && (
          <p className="notice" role="alert">
            {error}
          </p>
        )}
        <VirtualRows
          viewport={viewportRef}
          count={filteredEntityFieldList.length}
          list={virtualizer}
          renderRow={(index) => {
            const row = filteredEntityFieldList[index];
            if (!row) return null;
            const write = row.inner.write;
            const expanded =
              expandedFieldGroups.has(row.joinedNamedPath) ||
              visibleFieldGroups.has(row.joinedNamedPath);
            return (
              <div
                data-stripe={index % 2 === 1 || undefined}
                className={
                  "sheet-grid sheet-row" +
                  (row.expandableKind ? "" : " scalar-field-row")
                }
              >
                <FieldCells
                  row={row}
                  expanded={expanded}
                  comparing={comparing || !!write?.showTransition}
                  previousValue={
                    write?.before ?? prior.get(row.joinedNamedPath)?.inner.value
                  }
                  partial={partial}
                  removed={!current.has(row.joinedNamedPath)}
                  onToggle={handleFieldGroupToggle}
                  onFollow={onFollow}
                  onSource={
                    onSource &&
                    !row.expandableKind &&
                    current.has(row.joinedNamedPath)
                      ? () => onSource(row.inner.path, row.joinedNamedPath)
                      : undefined
                  }
                />
              </div>
            );
          }}
        />
      </SheetTable>
    </div>
  );
}
