import { StringTableRows } from "../stringTables/StringTableRows";
import { FilterToggle, SearchBar } from "../../workbench/PaneControls";
import { useState } from "react";
import type { Field } from "./MessageFields";
import { child } from "./packetStringTables";

export function PacketTableEntries({
  table,
  error,
}: {
  table?: Field;
  error?: string;
}) {
  const [query, setQuery] = useState("");
  const [client, setClient] = useState(false);
  const [limit, setLimit] = useState(128);
  const serverEntries = child(table, "items")?.children ?? [];
  const clientEntries = child(table, "items_clientside")?.children ?? [];
  const showClient =
    client || (!serverEntries.length && !!clientEntries.length);
  const rows = (showClient ? clientEntries : serverEntries).map((entry) => {
    const key = child(entry, "str")?.value;
    const data = child(entry, "data")?.value;
    return {
      index: Number(entry.name),
      keyText: key,
      dataSize: Number.parseInt(data ?? "") || undefined,
      data,
    };
  });
  return (
    <>
      <SearchBar
        label="Filter entries"
        value={query}
        onChange={(value) => {
          setQuery(value);
          setLimit(128);
        }}
      >
        {!!serverEntries.length && !!clientEntries.length && (
          <FilterToggle
            pressed={showClient}
            onClick={() => {
              setClient(!showClient);
              setLimit(128);
            }}
          >
            Client-side
          </FilterToggle>
        )}
      </SearchBar>
      <StringTableRows
        rows={rows}
        renderCells={(row) => ({ keyCell: row.keyText, dataCell: row.data })}
        query={query}
        error={error}
        limit={limit}
        showMore={() => setLimit(limit + 128)}
      />
    </>
  );
}
