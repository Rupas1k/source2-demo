import { branchKind } from "./messageBranches";
import { nextRecordIndex } from "../../workbench/recordNavigation";
import { SortLabel } from "../../workbench/FieldColumnHeader";
import { MessageBranchRows } from "./MessageBranchRows";
import { ChevronDown, ChevronRight } from "../../shared/components/Icons";
import { VirtualRecordList } from "../../workbench/VirtualRecordList";
import { FilterToggle, SearchBar } from "../../workbench/PaneControls";
import { IndexHeading } from "../../workbench/SheetTable";
import { messageKey } from "./useMessageBranch";
import { snapshotTables } from "./packetStringTables";

import { isFullPacket } from "./messageIndex";

import type { useMessageNavigation } from "./useMessageNavigation";
function MessageDisclosure({
  label,
  expanded,
  onToggle,
}: {
  label: string;
  expanded: boolean;
  onToggle: () => void;
}) {
  const Icon = expanded ? ChevronDown : ChevronRight;
  return (
    <button
      type="button"
      className="field-disclosure"
      aria-label={label}
      title={label}
      aria-expanded={expanded}
      onClick={onToggle}
    >
      <Icon size={13} aria-hidden="true" />
    </button>
  );
}

export function MessageList({
  wide,
  results,
  localRows,
  selected,
  activateMessage,
  navigateWide,
  selectMessage,
  list,
  query,
  term,
  matchedCommands,
  setQuery,
  setWide,
  viewport,
  indexSort,
  count,
  updateVisible,
  wideEntry,
  hasChildren,
  entityChanges,
  stringTables,
  gameEvents,
  isBranchOpen,
  selectedKey,
  selectedHasNoEntityEntries,
  details,
  entityFocus,
  tableFocus,
  eventFocus,
  isPacketOpen,
  togglePacket,
  toggleBranch,
  inspector,
  branchRow,
  changedEntityIndex,
  selectChangedEntity,
  selectPacketTable,
  selectPacketEvent,
}: ReturnType<typeof useMessageNavigation>["listProps"]) {
  const sortLabel = (column: "name" | "id" | "tick", label: string) => (
    <SortLabel
      label={label}
      direction={
        indexSort.sort?.column === column ? indexSort.sort.direction : undefined
      }
      onSort={() => {
        indexSort.toggleSort(column);
        viewport.current?.scrollTo({ top: 0 });
      }}
    />
  );
  return (
    <div
      className="sheet outline-none"
      tabIndex={0}
      onKeyDown={(event) => {
        if (event.defaultPrevented || event.altKey) return;
        if (
          event.key === "Enter" &&
          event.target === event.currentTarget &&
          selected
        ) {
          event.preventDefault();
          activateMessage(selected);
          return;
        }
        if (
          !(wide ? results?.rows : localRows.length) ||
          (event.key !== "ArrowUp" && event.key !== "ArrowDown")
        )
          return;
        event.preventDefault();
        if (wide) {
          void navigateWide(event.key === "ArrowDown");
          return;
        }
        const current = localRows.findIndex(
          (entry) =>
            entry.command === selected?.command &&
            entry.child === selected.child,
        );
        const next = nextRecordIndex(
          localRows.length,
          current,
          event.key === "ArrowDown",
        );
        selectMessage(localRows[next]);
        list.current?.scrollToIndex(next, { align: "nearest" });
      }}
    >
      <SearchBar
        label={wide ? "Search replay messages" : "Filter messages at this tick"}
        placeholder={wide ? "Search replay…" : "Filter this tick…"}
        value={query}
        matchCount={
          wide ? results?.matches : term ? matchedCommands.hits.size : undefined
        }
        onChange={setQuery}
      >
        <FilterToggle
          pressed={wide}
          onClick={() => {
            setWide(!wide);
            viewport.current?.scrollTo({ top: 0 });
          }}
        >
          All ticks
        </FilterToggle>
      </SearchBar>
      <IndexHeading
        label={sortLabel("name", "Message")}
        value={wide ? sortLabel("tick", "Tick") : sortLabel("id", "ID")}
      />
      <VirtualRecordList
        viewport={viewport}
        list={list}
        count={count}
        onRangeChange={updateVisible}
        renderRow={(index) => {
          const entry = wide ? wideEntry(index) : localRows[index];
          if (!entry) return null;
          const key = messageKey(entry);
          const parent = entry.child === null && hasChildren(entry);
          const kind = branchKind(entry);
          const entityBranch = entityChanges.results.get(key);
          const tables = stringTables.results.get(key);
          const events = gameEvents.results.get(key);
          const branchOpen = kind !== undefined && isBranchOpen(entry, kind);
          const branchLabel =
            kind === "entities"
              ? "Toggle entity changes"
              : kind === "stringTables"
                ? "Toggle string table changes"
                : "Toggle game events";
          const branch =
            kind === "entities"
              ? entityBranch
              : kind === "stringTables"
                ? tables
                : events;
          const canOpenBranch =
            kind !== undefined &&
            (branch
              ? !!(branch.changes.length || branch.error)
              : kind === "entities" && selectedKey === key
                ? !selectedHasNoEntityEntries
                : kind === "stringTables" &&
                    entry.child === null &&
                    selectedKey === key
                  ? !!snapshotTables(entry.name, details.value).length
                  : true);
          const isSelected =
            selectedKey === key && !entityFocus && !tableFocus && !eventFocus;
          return (
            <div key={key} className="message-list-item" data-index={index}>
              <div
                className={
                  "record-row record-row__selectable message-list-row" +
                  (isSelected ? " record-row__selected" : "")
                }
                data-depth={entry.child === null ? 0 : 1}
              >
                <div className="message-disclosures">
                  {parent || (isFullPacket(entry) && canOpenBranch) ? (
                    <MessageDisclosure
                      label={"Expand or collapse " + entry.name}
                      expanded={isPacketOpen(entry)}
                      onToggle={() => togglePacket(entry)}
                    />
                  ) : canOpenBranch && kind ? (
                    <MessageDisclosure
                      label={branchLabel}
                      expanded={branchOpen}
                      onToggle={() => toggleBranch(entry, kind)}
                    />
                  ) : null}
                </div>
                <button
                  className="message-select"
                  onClick={() => activateMessage(entry)}
                >
                  <span
                    className="truncate"
                    title={[
                      entry.name,
                      `Command ${entry.command}`,
                      entry.child === null
                        ? ""
                        : `Packet message ${entry.child}`,
                      isSelected && !details.error && inspector
                        ? `${inspector.payloadSize().toLocaleString()} bytes`
                        : "",
                    ]
                      .filter(Boolean)
                      .join("\n")}
                  >
                    {entry.name}
                  </span>
                  <span className="entity-id">
                    {wide ? entry.tick : entry.id}
                  </span>
                </button>
              </div>
              {branchOpen && canOpenBranch && kind === "entities" && (
                <MessageBranchRows
                  branch={entityBranch}
                  label="Affected entities"
                  loading="Loading entity changes…"
                  depth={2}
                  activeRef={branchRow}
                  describe={(change, index) => {
                    const active =
                      entityFocus?.key === key && index === changedEntityIndex;
                    return {
                      name: change.class,
                      id: change.index,
                      status: change.operation,
                      active,
                      select: () => {
                        selectMessage(entry);
                        if (!active) selectChangedEntity({ key, index });
                      },
                    };
                  }}
                />
              )}
              {branchOpen && canOpenBranch && kind === "stringTables" && (
                <MessageBranchRows
                  branch={tables}
                  label="Affected string tables"
                  loading="Loading string tables…"
                  depth={entry.child === null ? 1 : 2}
                  activeRef={branchRow}
                  describe={(table) => {
                    const active =
                      tableFocus?.key === key && tableFocus.name === table.name;
                    return {
                      name: table.name,
                      id: table.id,
                      active,
                      select: () => {
                        selectMessage(entry);
                        if (!active)
                          selectPacketTable({
                            key,
                            name: table.name,
                          });
                      },
                    };
                  }}
                />
              )}
              {branchOpen && canOpenBranch && kind === "gameEvents" && (
                <MessageBranchRows
                  branch={events}
                  label="Game events in message"
                  loading="Loading game events…"
                  depth={2}
                  activeRef={branchRow}
                  describe={(event) => {
                    const active =
                      eventFocus?.key === key && eventFocus.id === event.id;
                    return {
                      name: event.name,
                      id: event.id,
                      active,
                      select: () => {
                        selectMessage(entry);
                        if (!active)
                          selectPacketEvent({
                            key,
                            id: event.id,
                            name: event.name,
                            definition: event.definition,
                          });
                      },
                    };
                  }}
                />
              )}
            </div>
          );
        }}
      />
    </div>
  );
}
