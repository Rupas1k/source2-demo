import { useTableSort } from "../../workbench/tableSort";
import { useMemo } from "react";
import { readEventKeys } from "../../replay/readGameEvents";
import { useReplayStore } from "../../replay/state";
import { EventFieldTable } from "./EventFieldTable";

export function GameEventKeyList({
  eventId,
  query,
}: {
  eventId?: number;
  query: string;
}) {
  const session = useReplayStore((state) => state.replaySession);
  const tick = useReplayStore((state) => state.currentTick);
  const rows = useMemo(() => {
    void tick;
    return readEventKeys(session, eventId);
  }, [session, eventId, tick]);
  const fieldSort = useTableSort();
  return (
    <EventFieldTable
      sortControl={fieldSort}
      rows={rows}
      query={query}
      definition
    />
  );
}
