import { FilterToggle } from "../../workbench/PaneControls";
import { useEffect, useMemo, useState } from "react";
import { RecordIndex } from "../../workbench/RecordIndex";
import { RecordPanes } from "../../workbench/RecordPanes";
import { EntityFieldList } from "./EntityFieldList";
import { useReplayActivity } from "../../replay/useReplayActivity";
import { useReplayStore } from "../../replay/state";
export default function EntitiesPanel() {
  const parser = useReplayStore((state) => state.replaySession);
  const tick = useReplayStore((state) => state.currentTick);
  const entity = useReplayStore((state) => state.selectedEntity);
  const selectEntity = useReplayStore((state) => state.setSelectedEntity);
  const baseline = useReplayStore((state) => state.selectedBaseline);
  const selectBaseline = useReplayStore((state) => state.setSelectedBaseline);
  const view = useReplayStore((state) => state.entityView);
  const setView = useReplayStore((state) => state.setActiveView);
  const baselines = view === "baselineEntities";
  const selected = baselines ? baseline : entity;
  const [target, reveal] = useState<{ index: number } | undefined>(() =>
    selected === undefined ? undefined : { index: selected },
  );
  const records = useMemo(() => {
    const entries = baselines
      ? parser?.listBaselineEntities()
      : parser?.listEntities();
    return (entries ?? []).map((entry) => {
      const row = { key: entry.index, id: entry.index, name: entry.name };
      entry.free();
      return row;
    });
  }, [parser, tick, baselines]);
  const [lifetime, setLifetime] = useState<{
    index: number;
    handle: number;
    name: string;
  }>();
  useEffect(() => {
    if (!parser || entity === undefined) {
      setLifetime(undefined);
      return;
    }
    setLifetime((current) => {
      if (current?.index === entity) return current;
      const handle = parser.entityHandle(entity);
      if (handle === undefined) return undefined;
      const name = !baselines
        ? records.find((row) => row.id === entity)?.name
        : undefined;
      return { index: entity, handle, name: name ?? "Entity" };
    });
  }, [baselines, entity, parser, records, tick]);
  const tracked = lifetime?.index === entity ? lifetime : undefined;
  const baselineName = baselines
    ? records.find((row) => row.id === baseline)?.name
    : undefined;
  useReplayActivity(
    baselines ? { view, classId: baseline } : { view, handle: tracked?.handle },
    baselines
      ? baseline === undefined
        ? "Baseline writes"
        : `Baseline writes · ${baselineName ?? baseline}`
      : tracked
        ? `Entity activity · ${tracked.name} ${tracked.index}`
        : "Entity activity",
  );
  return (
    <RecordPanes
      recordTitle="Class name"
      detailTitle="Properties"
      records={
        <RecordIndex
          records={records}
          selected={selected}
          onSelect={
            baselines
              ? selectBaseline
              : (index) => {
                  const handle =
                    index === undefined
                      ? undefined
                      : parser?.entityHandle(index);
                  setLifetime(
                    index !== undefined && handle !== undefined
                      ? {
                          index,
                          handle,
                          name:
                            records.find((row) => row.id === index)?.name ??
                            "Entity",
                        }
                      : undefined,
                  );
                  selectEntity(index);
                }
          }
          filterAction={
            <FilterToggle
              pressed={baselines}
              onClick={() => {
                const next = baselines ? entity : baseline;
                reveal(next === undefined ? undefined : { index: next });
                setView(baselines ? "entities" : "baselineEntities");
              }}
            >
              Baselines
            </FilterToggle>
          }
          heading="Class name"
          filterLabel={baselines ? "Filter baselines" : "Filter entities"}
          reveal={target}
        />
      }
      details={<EntityFieldList reveal={reveal} />}
    />
  );
}
