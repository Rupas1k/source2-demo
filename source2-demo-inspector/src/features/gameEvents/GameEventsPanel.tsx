import { sortRows, useTableSort } from "../../workbench/tableSort";
import { FilterToggle, SearchField } from "../../workbench/PaneControls";
import { useEffect, useMemo, useState } from "react";
import type { ReplaySession } from "../../generated/wasm/inspector";
import { useReplayStore } from "../../replay/state";
import { RecordIndex } from "../../workbench/RecordIndex";
import { RecordPanes } from "../../workbench/RecordPanes";
import { GameEventKeyList } from "./GameEventKeyList";

import {
  readEventDefinitions,
  readFiredEvents,
  type FiredEvent,
} from "../../replay/readGameEvents";
import { EventFieldTable } from "./EventFieldTable";

type Column = "name" | "tick" | "id";

export default function GameEventsPanel() {
  const isActive = useReplayStore((state) => state.activeView === "gameEvents");
  const setActivity = useReplayStore((state) => state.setActivity);
  const openSourceMessage = useReplayStore((state) => state.openSourceMessage);
  const setView = useReplayStore((state) => state.setActiveView);
  const [definitions, showDefinitions] = useState(false);
  const definition = useReplayStore((state) => state.selectedEvent);
  const selectDefinition = useReplayStore((state) => state.setSelectedEvent);
  const [definitionFilter, setDefinitionFilter] = useState("");
  const [definitionQuery, setDefinitionQuery] = useState("");
  const session = useReplayStore((state) => state.replaySession);
  const parserTick = useReplayStore((state) => state.currentTick);
  const eventTick = useReplayStore((state) => state.selectedEventTick);
  const setEventTick = useReplayStore((state) => state.setSelectedEventTick);
  const tick = eventTick ?? parserTick;
  const [eventQuery, setEventQuery] = useState("");
  const [fieldQuery, setFieldQuery] = useState("");
  const [allTicks, setAllTicks] = useState(false);
  const sortControl = useTableSort<Column>({
    column: "tick",
    direction: "ascending",
  });
  const { sort } = sortControl;
  const fieldSort = useTableSort();
  const [picked, pick] = useState<number>();
  const [loaded, setLoaded] = useState<{
    session: ReplaySession;
    events: FiredEvent[];
    error?: string;
  }>();
  useEffect(() => {
    if (!session) return;
    if (useReplayStore.getState().selectedEventTick === undefined) {
      setEventTick(useReplayStore.getState().currentTick);
    }
    try {
      setLoaded({ session, events: readFiredEvents(session) });
    } catch (error) {
      setLoaded({ session, events: [], error: String(error) });
    }
  }, [session, setEventTick]);
  const schemas = useMemo(() => {
    void parserTick;
    return readEventDefinitions(session);
  }, [session, parserTick]);
  const active = loaded?.session === session ? loaded : undefined;
  function showSource(event: FiredEvent) {
    openSourceMessage(event, () => {
      pick(picked);
      setAllTicks(allTicks);
      setEventQuery(eventQuery);
      setFieldQuery(fieldQuery);
      showDefinitions(definitions);
      setView("gameEvents");
    });
  }

  const records = useMemo(() => {
    const rows = (active?.events ?? []).flatMap((event, key) =>
      allTicks || event.tick === tick
        ? [
            {
              key,
              id: event.id,
              name: event.name,
              tick: event.tick,
            },
          ]
        : [],
    );
    return allTicks ? sortRows(rows, sort, (row, column) => row[column]) : rows;
  }, [active, allTicks, sort, tick]);
  const pickedEvent = picked === undefined ? undefined : active?.events[picked];
  const event =
    pickedEvent && (allTicks || pickedEvent.tick === tick)
      ? pickedEvent
      : undefined;
  const selected = event ? picked : undefined;
  const activityId = definitions ? definition : event?.id;
  const activityName = definitions
    ? schemas.find((schema) => schema.id === definition)?.name
    : event?.name;
  const scope = (definitions ? definitionFilter : eventQuery)
    .trim()
    .toLowerCase();
  const matchByTick = !definitions && allTicks;
  useEffect(() => {
    if (!isActive || !active) return;
    const occurrences = active.events.flatMap((entry, key) => {
      const matches = scope
        ? entry.name.toLowerCase().includes(scope) ||
          String(entry.id).includes(scope) ||
          (matchByTick && String(entry.tick) === scope)
        : activityId === undefined || entry.id === activityId;
      return matches ? [{ tick: entry.tick, key }] : [];
    });
    setActivity("gameEvents", {
      label: scope
        ? `Events matching “${scope}”`
        : (activityName ?? "Fired events"),
      points: occurrences.map((entry) => ({ tick: entry.tick, count: 1 })),
      jump:
        definitions || (!scope && activityId === undefined)
          ? undefined
          : (tick) => {
              const occurrence = occurrences.find(
                (entry) => entry.tick === tick,
              );
              if (!occurrence) return;
              pick(occurrence.key);
              setEventTick(tick);
            },
    });
  }, [
    active,
    activityId,
    activityName,
    definitions,
    isActive,
    matchByTick,
    scope,
    setActivity,
    setEventTick,
  ]);
  return (
    <RecordPanes
      recordTitle={definitions ? "Event definitions" : "Fired events"}
      detailTitle={definitions ? "Key schema" : "Event values"}
      records={
        <>
          <div className="pane-layout" hidden={definitions}>
            <RecordIndex
              records={records}
              selected={selected}
              heading="Event"
              filterLabel={allTicks ? "Search replay" : "Filter this tick"}
              onQueryChange={setEventQuery}
              onActivate={(key) => {
                const occurrence = active?.events[key];
                if (occurrence) showSource(occurrence);
              }}
              filterAction={
                <FilterToggle
                  pressed={allTicks}
                  onClick={() => setAllTicks(!allTicks)}
                >
                  All ticks
                </FilterToggle>
              }
              searchIds
              tickColumn={allTicks}
              sortControl={allTicks ? sortControl : undefined}
              onSelect={(key) => {
                pick(key);
                if (key === undefined) return;
                const event = active?.events[key];
                if (allTicks && event) setEventTick(event.tick);
              }}
            />
          </div>
          <div className="pane-layout" hidden={!definitions}>
            <RecordIndex
              records={schemas}
              selected={definition}
              onSelect={selectDefinition}
              heading="Event"
              filterLabel="Filter definitions"
              onQueryChange={setDefinitionFilter}
              searchIds
            />
          </div>
        </>
      }
      details={
        <div className="sheet">
          <div className="field-tools">
            <SearchField
              label="Filter keys"
              value={definitions ? definitionQuery : fieldQuery}
              onChange={definitions ? setDefinitionQuery : setFieldQuery}
            />
            {!definitions && event && (
              <button type="button" onClick={() => showSource(event)}>
                Source message
              </button>
            )}
            {!definitions && event && (
              <span className="event-detail-tick">
                Tick {event.tick.toLocaleString()}
              </span>
            )}
            <FilterToggle
              pressed={definitions}
              onClick={() => {
                if (
                  !definitions &&
                  definition === undefined &&
                  event &&
                  schemas.some((schema) => schema.id === event.id)
                ) {
                  selectDefinition(event.id);
                }
                showDefinitions(!definitions);
              }}
            >
              Definitions
            </FilterToggle>
          </div>
          {definitions ? (
            <GameEventKeyList eventId={definition} query={definitionQuery} />
          ) : (
            <EventFieldTable
              sortControl={fieldSort}
              rows={event?.fields}
              query={fieldQuery}
              error={active?.error}
            />
          )}
        </div>
      }
    />
  );
}
