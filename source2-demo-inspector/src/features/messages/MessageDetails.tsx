import { PacketGameEventFields } from "./PacketGameEventFields";
import { useTableSort } from "../../workbench/tableSort";
import { SearchBar } from "../../workbench/PaneControls";
import { SheetTable } from "../../workbench/SheetTable";
import {
  FieldRows,
  filterMessageFields,
  sortMessageFields,
} from "./MessageFields";
import { EntityChangeFields } from "./EntityChangeFields";
import { PacketTableEntries } from "./PacketStringTablesPanel";

import type { useMessageNavigation } from "./useMessageNavigation";
export function MessageDetails({
  fieldQuery,
  setFieldQuery,
  eventFocus,
  gameEvents,
  linkError,
  tableFocus,
  inspector,
  tableDetails,
  tableBranch,
  entityFocus,
  changedEntityIndex,
  changedEntity,
  selectedBranch,
  openEntityView,
  problem,
  selected,
  details,
}: ReturnType<typeof useMessageNavigation>["detailsProps"]) {
  const fieldSort = useTableSort();
  return eventFocus ? (
    <PacketGameEventFields
      key={eventFocus.key + "/" + eventFocus.id}
      event={gameEvents.results
        .get(eventFocus.key)
        ?.changes.find((event) => event.id === eventFocus.id)}
      error={gameEvents.results.get(eventFocus.key)?.error || linkError}
    />
  ) : tableFocus && inspector ? (
    <div className="sheet">
      <PacketTableEntries
        key={tableFocus.key + "/" + tableFocus.name}
        table={tableDetails?.value}
        error={tableDetails?.error || tableBranch?.error || linkError}
      />
    </div>
  ) : entityFocus ? (
    <EntityChangeFields
      key={entityFocus.key + "/" + changedEntityIndex}
      change={changedEntity}
      targetPath={entityFocus.target?.path.join(".")}
      error={
        linkError ||
        selectedBranch?.error ||
        (selectedBranch && !changedEntity
          ? "The source field change is unavailable."
          : undefined)
      }
      openEntity={(change) => openEntityView(change.handle, false)}
      openLinkedEntity={(handle) => openEntityView(handle, false)}
      openBaseline={(change) => openEntityView(change.class_id, true)}
    />
  ) : (
    <div className="sheet">
      <SearchBar
        label="Filter fields"
        value={fieldQuery}
        onChange={setFieldQuery}
      />
      <SheetTable
        tableId="message-fields"
        className="field-tree"
        columns={["Field", "Type", "Value"]}
        sort={fieldSort.sort}
        onSort={fieldSort.toggleSort}
      >
        {problem && (
          <p role="alert" className="notice">
            {problem}
          </p>
        )}
        {inspector && (
          <FieldRows
            key={
              selected?.command +
              "/" +
              selected?.child +
              "/" +
              fieldQuery.trim() +
              "/" +
              fieldSort.sort?.column +
              "/" +
              fieldSort.sort?.direction
            }
            fields={sortMessageFields(
              filterMessageFields(details.value, fieldQuery),
              fieldSort.sort,
            )}
          />
        )}
      </SheetTable>
    </div>
  );
}
