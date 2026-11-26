import { seekReplay } from "../../replay/seekReplay";
import { FilterToggle } from "../../workbench/PaneControls";
import { EntityFieldTable, useEntityFieldTable } from "./EntityFieldTable";
import { readFields } from "../../replay/readFields";
import { useLayoutEffect, useMemo, useRef } from "react";

import {
  useReplayStore,
  selectInspectedEntity,
  type MessageLocation,
} from "../../replay/state";

type FieldSource =
  | { kind: "creation" | "update"; location: MessageLocation }
  | { kind: "baseline"; classId: number; tick: number };

export function EntityFieldList({
  reveal,
}: {
  reveal: (target: { index: number }) => void;
}) {
  const session = useReplayStore((state) => state.replaySession);
  const view = useReplayStore((state) => state.entityView);
  const setView = useReplayStore((state) => state.setActiveView);
  const entityIndex = useReplayStore(selectInspectedEntity);
  const tick = useReplayStore((state) => state.currentTick);
  const setPosition = useReplayStore((state) => state.setReplayPosition);

  const rawEntityFieldList = useMemo(
    () => readFields(session, view, entityIndex),
    [session, view, entityIndex, tick],
  );

  const comparison = useReplayStore((state) => state.comparison);
  const previous =
    comparison?.view === view && comparison.index === entityIndex
      ? comparison
      : undefined;
  const table = useEntityFieldTable(rawEntityFieldList, previous);
  const {
    query,
    setQuery,
    changes,
    setChanges,
    viewportRef,
    expandedFieldGroups,
    setExpandedFieldGroups,
  } = table;

  const selectEntity = useReplayStore((state) => state.setSelectedEntity);
  const selectBaseline = useReplayStore((state) => state.setSelectedBaseline);
  const openSourceMessage = useReplayStore((state) => state.openSourceMessage);
  const setTimelineError = useReplayStore((state) => state.setTimelineError);
  const pushBack = useReplayStore((state) => state.pushBack);
  const pendingScroll = useRef<number>(undefined);
  useLayoutEffect(() => {
    if (pendingScroll.current !== undefined && viewportRef.current) {
      viewportRef.current.scrollTop = pendingScroll.current;
      pendingScroll.current = undefined;
    }
  });
  function followEntity(index: number) {
    if (
      entityIndex === undefined ||
      (view === "entities" && entityIndex === index)
    )
      return;
    const scroll = viewportRef.current?.scrollTop ?? 0;
    pushBack(() => {
      setView(view);
      (view === "baselineEntities" ? selectBaseline : selectEntity)(
        entityIndex,
      );
      reveal({ index: entityIndex });
      setQuery(query);
      setChanges(changes);
      setExpandedFieldGroups(expandedFieldGroups);
      pendingScroll.current = scroll;
    });
    setQuery("");
    setChanges(false);
    setExpandedFieldGroups(new Set());
    pendingScroll.current = 0;
    setView("entities");

    selectEntity(index);
    reveal({ index });
  }
  function openFieldSource(path: Uint16Array, name: string) {
    if (!session || entityIndex === undefined) return;
    try {
      const source: FieldSource | null = JSON.parse(
        session.findFieldSource(entityIndex, path),
      );
      if (!source) {
        setTimelineError(
          `No recorded field write could be traced for ${name}.`,
        );
        return;
      }
      const scroll = viewportRef.current?.scrollTop ?? 0;
      const restore = () => {
        setView("entities");
        selectEntity(entityIndex);
        reveal({ index: entityIndex });
        setQuery(query);
        setChanges(changes);
        setExpandedFieldGroups(expandedFieldGroups);
        pendingScroll.current = scroll;
      };
      if (source.kind !== "baseline") {
        const handle = session.entityHandle(entityIndex);
        const { child } = source.location;
        if (handle === undefined || child === null)
          throw new Error("The source entity change is unavailable.");
        openSourceMessage(source.location, restore, {
          kind: source.kind,
          handle,
          path: Array.from(path),
        });
        return;
      }
      const actual = seekReplay(session, source.tick);
      const fields = readFields(session, "baselineEntities", source.classId);
      const hasField = fields?.some((field) => {
        const candidate = field.path;
        return (
          candidate.length === path.length &&
          candidate.every((part, index) => part === path[index])
        );
      });
      if (!hasField) {
        seekReplay(session, tick);
        setTimelineError(
          `${name} was inherited from the class baseline at tick ${source.tick}.`,
        );
        return;
      }
      pushBack(restore);
      setPosition(actual, undefined);
      setQuery(name);
      setChanges(false);
      selectBaseline(source.classId);
      setView("baselineEntities");
      reveal({ index: source.classId });
    } catch (error) {
      setTimelineError(error instanceof Error ? error.message : String(error));
    }
  }
  return (
    <EntityFieldTable
      table={table}
      onFollow={followEntity}
      onSource={view === "entities" ? openFieldSource : undefined}
      actions={
        previous && (
          <FilterToggle
            pressed={changes}
            onClick={() => setChanges(!changes)}
            title={`Compare with tick ${previous.tick}`}
          >
            Changes
          </FilterToggle>
        )
      }
    />
  );
}
