import { useTableSort } from "../../workbench/tableSort";
import { useState } from "react";
import { SearchField } from "../../workbench/PaneControls";
import { EventFieldTable } from "../gameEvents/EventFieldTable";
import type { PacketGameEvent } from "./packetGameEvents";

export function PacketGameEventFields({
  event,
  error,
}: {
  event?: PacketGameEvent;
  error?: string;
}) {
  const [query, setQuery] = useState("");
  const fieldSort = useTableSort();
  return (
    <div className="sheet">
      <div className="field-tools">
        <SearchField label="Filter keys" value={query} onChange={setQuery} />
      </div>
      <EventFieldTable
        sortControl={fieldSort}
        rows={
          event?.definition
            ? event.fields.map((field, index) => ({ ...field, index }))
            : event?.fields
        }
        query={query}
        definition={event?.definition}
        error={error}
      />
    </div>
  );
}
