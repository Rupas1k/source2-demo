import { useMemo, useState } from "react";
import { RecordIndex } from "../../workbench/RecordIndex";
import { RecordPanes } from "../../workbench/RecordPanes";
import { StringTableItemList } from "./StringTableItemList";
import { useReplayActivity } from "../../replay/useReplayActivity";
import { useReplayStore } from "../../replay/state";
export default function StringTablesPanel() {
  const parser = useReplayStore((state) => state.replaySession);
  const tick = useReplayStore((state) => state.currentTick);
  const selected = useReplayStore((state) => state.selectedTable);
  const select = useReplayStore((state) => state.setSelectedTable);
  const records = useMemo(() => {
    const entries = parser?.listStringTables();
    return (entries ?? []).map((entry) => {
      const row = { key: entry.name, id: entry.index, name: entry.name };
      entry.free();
      return row;
    });
  }, [parser, tick]);
  const [row, selectRow] = useState<{ table: string; index: number }>();
  const selectedRow = row?.table === selected ? row?.index : undefined;
  useReplayActivity(
    { view: "stringTables", table: selected, row: selectedRow },
    selected === undefined
      ? "String-table writes"
      : selectedRow === undefined
        ? `String table writes for ${selected}`
        : `String table writes for ${selected}, row ${selectedRow}`,
  );
  return (
    <RecordPanes
      recordTitle="Table"
      detailTitle="Entries"
      records={
        <RecordIndex
          records={records}
          selected={selected}
          onSelect={select}
          heading="Table"
          filterLabel="Filter tables"
        />
      }
      details={
        <StringTableItemList
          key={selected}
          tableName={selected}
          selectedRow={selectedRow}
          onSelectRow={(index) => {
            if (selected !== undefined)
              selectRow(
                index === selectedRow ? undefined : { table: selected, index },
              );
          }}
        />
      }
    />
  );
}
