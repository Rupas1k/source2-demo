import { useEffect } from "react";
import type { ActivityQuery } from "./activityQuery";
import { requestReplayActivity } from "./replayActivity";
import { useReplayStore } from "./state";

export function useReplayActivity(query: ActivityQuery, label: string) {
  const view = query.view;
  const active = useReplayStore((state) => state.activeView === view);
  const file = useReplayStore((state) => state.replayFile);
  const parser = useReplayStore((state) => state.replaySession);
  const setActivity = useReplayStore((state) => state.setActivity);
  const handle = view === "entities" ? query.handle : undefined;
  const classId = view === "baselineEntities" ? query.classId : undefined;
  const table = view === "stringTables" ? query.table : undefined;
  const row = view === "stringTables" ? query.row : undefined;

  useEffect(() => {
    if (!file || !parser || !active) return;
    setActivity(view);
    let cancelled = false;
    const request =
      view === "entities"
        ? ({ view, handle } as const)
        : view === "baselineEntities"
          ? ({ view, classId } as const)
          : ({ view, table, row } as const);
    void requestReplayActivity(file, request)
      .then((points) => {
        if (!cancelled) setActivity(view, { label, points });
      })
      .catch(() => {
        // Leave the standard timeline available if activity indexing fails.
      });
    return () => {
      cancelled = true;
    };
  }, [
    active,
    classId,
    file,
    handle,
    label,
    parser,
    row,
    setActivity,
    table,
    view,
  ]);
}
