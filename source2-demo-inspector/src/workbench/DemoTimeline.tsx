import { captureComparison, seekReplay } from "../replay/seekReplay";
import { ActivityStrip } from "./ActivityStrip";
import {
  ChevronLeft,
  ChevronRight,
  SkipBack,
  SkipForward,
} from "../shared/components/Icons";
import { useEffect, useMemo, useRef, useState } from "react";
import { useReplayStore, selectInspectedEntity } from "../replay/state";
import { Slider } from "../shared/components/Slider";
import { formatNumber } from "../shared/utils/formatNumber";

const minimumZoomSpan = 16;

function centeredStart(tick: number, span: number, last: number) {
  return Math.max(0, Math.min(last - span, tick - Math.floor(span / 2)));
}

export default function DemoTimeline() {
  const parser = useReplayStore((state) => state.replaySession);
  const view = useReplayStore((state) => state.activeView);
  const activity = useReplayStore((state) => state.activity[state.activeView]);
  const index = useReplayStore(selectInspectedEntity);
  const selectedTable = useReplayStore((state) => state.selectedTable);
  const parserTick = useReplayStore((state) => state.currentTick);
  const setPosition = useReplayStore((state) => state.setReplayPosition);
  const ticks = useReplayStore((state) => state.messageTicks);
  const messageTick = useReplayStore((state) => state.selectedMessageTick);
  const setMessageTick = useReplayStore(
    (state) => state.setSelectedMessageTick,
  );
  const eventTick = useReplayStore((state) => state.selectedEventTick);
  const setEventTick = useReplayStore((state) => state.setSelectedEventTick);
  const viewTick =
    view === "messages"
      ? messageTick
      : view === "gameEvents"
        ? eventTick
        : undefined;
  const tick = viewTick ?? parserTick;
  const [cursor, setCursor] = useState(tick);
  const [draft, setDraft] = useState<string>();
  const setError = useReplayStore((state) => state.setTimelineError);
  const last = Math.max(0, parser?.totalTicks() ?? 0);
  const [zoom, setZoom] = useState<{ start: number; span: number }>();
  const span = zoom?.span ?? last;
  const start =
    zoom && tick >= zoom.start && tick <= zoom.start + span
      ? zoom.start
      : centeredStart(tick, span, last);
  const end = start + span;
  const tracks = useRef<HTMLDivElement>(null);
  const [previousOccurrence, nextOccurrence] = useMemo(() => {
    if (activity?.occurrence)
      return [activity.occurrence.previous, activity.occurrence.next];
    let previous: number | undefined;
    let next: number | undefined;
    for (const { tick: candidate } of activity?.points ?? []) {
      if (candidate < tick && (previous === undefined || candidate > previous))
        previous = candidate;
      if (candidate > tick && (next === undefined || candidate < next))
        next = candidate;
    }
    return [previous, next];
  }, [activity, tick]);

  useEffect(() => {
    if (zoom && start !== zoom.start) setZoom({ start, span });
  }, [span, start, zoom]);
  useEffect(() => setZoom(undefined), [parser]);

  useEffect(() => {
    const element = tracks.current;
    if (!parser || !element || last <= minimumZoomSpan) return;
    let delta = 0;
    let frame: number | undefined;

    function applyZoom() {
      frame = undefined;
      const factor = 2 ** (delta / 120);
      delta = 0;
      setZoom((current) => {
        const currentSpan = current?.span ?? last;
        const scaled = currentSpan * factor;
        const rounded = factor < 1 ? Math.floor(scaled) : Math.ceil(scaled);
        const nextSpan = Math.min(last, Math.max(minimumZoomSpan, rounded));
        if (nextSpan === currentSpan) return current;
        return nextSpan === last
          ? undefined
          : { start: centeredStart(tick, nextSpan, last), span: nextSpan };
      });
      setCursor(tick);
    }

    function onWheel(event: WheelEvent) {
      if (
        event.ctrlKey ||
        event.metaKey ||
        !event.deltaY ||
        Math.abs(event.deltaY) <= Math.abs(event.deltaX)
      )
        return;

      event.preventDefault();
      const unit = event.deltaMode === 1 ? 16 : event.deltaMode === 2 ? 120 : 1;
      delta += event.deltaY * unit;
      if (frame === undefined) frame = requestAnimationFrame(applyZoom);
    }

    element.addEventListener("wheel", onWheel, { passive: false });
    return () => {
      element.removeEventListener("wheel", onWheel);
      if (frame !== undefined) cancelAnimationFrame(frame);
    };
  }, [last, parser, tick]);

  useEffect(() => {
    setCursor(tick);
    setDraft(undefined);
    setError("");
  }, [tick, parser, setError]);

  function adjacent(forward: boolean) {
    if (!parser) return undefined;
    if (!ticks?.length) {
      const target = parser.adjacentTick(forward);
      return (forward ? target > tick : target < tick) ? target : undefined;
    }
    let lo = 0,
      hi = ticks.length;
    while (lo < hi) {
      const mid = lo + Math.floor((hi - lo) / 2);
      if (ticks[mid] < tick || (forward && ticks[mid] === tick)) lo = mid + 1;
      else hi = mid;
    }
    return ticks[forward ? lo : lo - 1];
  }

  function recorded(target: number, min = 0, max = last) {
    if (!ticks?.length) return target;
    let lo = 0,
      hi = ticks.length - 1;
    while (lo < hi) {
      const mid = Math.floor((lo + hi) / 2);
      if (ticks[mid] < target) lo = mid + 1;
      else hi = mid;
    }
    const previous = ticks[Math.max(0, lo - 1)];
    const next = ticks[lo];
    if (previous < min || previous > max)
      return next >= min && next <= max ? next : tick;
    if (next < min || next > max) return previous;
    return Math.abs(target - previous) < Math.abs(next - target)
      ? previous
      : next;
  }

  function seek(target: number) {
    if (!parser || !Number.isFinite(target)) return;
    setDraft(undefined);
    try {
      const requested = recorded(
        Math.min(last, Math.max(0, Math.round(target))),
      );
      if (view === "messages" || view === "gameEvents") {
        (view === "messages" ? setMessageTick : setEventTick)(requested);
        setCursor(requested);
        setError("");
        return;
      }
      const before = captureComparison(
        parser,
        tick,
        view,
        index,
        selectedTable,
      );
      const actual = seekReplay(parser, requested);
      if (actual !== tick) setPosition(actual, before);
      setCursor(actual);
      setError("");
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
    }
  }

  const previousTick = adjacent(false);
  const nextTick = adjacent(true);

  function stepRecorded(forward: boolean) {
    const target = forward ? nextTick : previousTick;
    if (target !== undefined) seek(target);
  }

  function occurrenceTitle(label: string, target: number | undefined) {
    if (target === undefined) return `No ${label.toLowerCase()}`;
    const destination = `${label} at tick ${formatNumber(target)}`;
    return activity ? `${destination}\n${activity.label}` : destination;
  }

  function stepOccurrence(forward: boolean) {
    if (activity?.occurrence) {
      activity.occurrence.step(forward);
      return;
    }
    const target = forward ? nextOccurrence : previousOccurrence;
    if (target !== undefined) (activity?.jump ?? seek)(target);
  }

  return (
    <section className="transport" aria-label="Timeline">
      <div className="transport-head">
        <div
          className="transport-controls"
          role="group"
          aria-label="Recorded ticks"
        >
          <label className="transport-tick">
            Tick{" "}
            <input
              className="transport-tick-value"
              type="text"
              inputMode="numeric"
              aria-label="Go to tick"
              disabled={!parser}
              value={draft ?? String(tick)}
              onChange={(event) => {
                const value = event.target.value.trim();
                if (/^\d*$/.test(value)) setDraft(value);
              }}
              onBlur={() => setDraft(undefined)}
              onKeyDown={(event) => {
                if (event.defaultPrevented || event.altKey) return;
                if (event.key === "Enter") {
                  event.preventDefault();
                  if (draft) seek(Number(draft));
                  else setDraft(undefined);
                } else if (event.key === "Escape" && draft !== undefined) {
                  event.preventDefault();
                  setDraft(undefined);
                } else if (
                  event.key === "ArrowUp" ||
                  event.key === "ArrowDown"
                ) {
                  event.preventDefault();
                  stepRecorded(event.key === "ArrowUp");
                }
              }}
            />
          </label>
          <button
            type="button"
            aria-label="Previous recorded tick"
            title={
              previousTick === undefined
                ? "No previous recorded tick"
                : `Previous recorded tick at ${formatNumber(previousTick)}`
            }
            disabled={previousTick === undefined}
            onClick={() => stepRecorded(false)}
          >
            <ChevronLeft size={16} />
          </button>
          <button
            type="button"
            aria-label="Next recorded tick"
            title={
              nextTick === undefined
                ? "No next recorded tick"
                : `Next recorded tick at ${formatNumber(nextTick)}`
            }
            disabled={nextTick === undefined}
            onClick={() => stepRecorded(true)}
          >
            <ChevronRight size={16} />
          </button>
        </div>
        <div
          className="transport-controls timeline-occurrences"
          role="group"
          aria-label="Timeline occurrences"
        >
          <button
            type="button"
            aria-label="Previous occurrence"
            title={occurrenceTitle("Previous occurrence", previousOccurrence)}
            disabled={!parser || previousOccurrence === undefined}
            onClick={() => stepOccurrence(false)}
          >
            <SkipBack size={15} />
          </button>
          <button
            type="button"
            aria-label="Next occurrence"
            title={occurrenceTitle("Next occurrence", nextOccurrence)}
            disabled={!parser || nextOccurrence === undefined}
            onClick={() => stepOccurrence(true)}
          >
            <SkipForward size={15} />
          </button>
        </div>
      </div>
      <div className="timeline-tracks" ref={tracks}>
        {parser && !!activity?.points.length && (
          <ActivityStrip
            activity={activity}
            tick={tick}
            start={start}
            end={end}
            jump={activity.jump ?? seek}
            step={stepOccurrence}
          />
        )}
        <div className="track">
          {cursor !== tick && cursor >= start && cursor <= end && (
            <output
              className="tick-preview"
              style={{
                left: `clamp(3em, ${(100 * (cursor - start)) / Math.max(1, span)}%, calc(100% - 3em))`,
              }}
            >
              {cursor.toLocaleString()}
            </output>
          )}
          <Slider
            className="seek-control"
            title={parser ? "Scroll to zoom the timeline" : undefined}
            disabled={!parser}
            min={start}
            max={end || 1}
            value={cursor >= start && cursor <= end ? cursor : tick}
            onValueChange={(value) => setCursor(recorded(value, start, end))}
            onValueCommit={(value) => seek(recorded(value, start, end))}
            onKeyDownCapture={(e) => {
              if (e.defaultPrevented || e.altKey) return;
              if (
                parser &&
                ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"].includes(
                  e.key,
                )
              ) {
                e.preventDefault();
                e.stopPropagation();
                stepRecorded(e.key === "ArrowRight" || e.key === "ArrowUp");
              }
            }}
          />
        </div>
      </div>
    </section>
  );
}
