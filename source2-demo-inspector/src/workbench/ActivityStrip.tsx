import { useLayoutEffect, useMemo, useRef, useState } from "react";
import type { TimelineActivity } from "../replay/state";

const numberFormat = new Intl.NumberFormat();

function lowerBound(
  ticks: Int32Array,
  target: number,
  start = 0,
  end = ticks.length,
) {
  while (start < end) {
    const mid = start + Math.floor((end - start) / 2);
    if (ticks[mid] < target) start = mid + 1;
    else end = mid;
  }
  return start;
}

export function ActivityStrip({
  activity,
  tick,
  start,
  end,
  jump,
  step,
}: {
  activity: TimelineActivity;
  tick: number;
  start: number;
  end: number;
  jump: (tick: number) => void;
  step: (forward: boolean) => void;
}) {
  const track = useRef<HTMLDivElement>(null);
  const [width, setWidth] = useState(0);
  useLayoutEffect(() => {
    const element = track.current!;
    const measure = () => setWidth(element.clientWidth);
    measure();
    const observer = new ResizeObserver(measure);
    observer.observe(element);
    return () => observer.disconnect();
  }, []);
  const span = Math.max(1, end - start);
  const showTicks = (span + 1) * 8 <= width;
  const count = Math.max(1, Math.min(400, Math.floor(width / 4)));
  // Index totals once so changes to the zoom range only visit the visible bins.
  const index = useMemo(() => {
    const points = [...activity.points].sort((a, b) => a.tick - b.tick);
    const ticks = Int32Array.from(points, (point) => point.tick);
    const matches = new Float64Array(points.length + 1);
    const created = new Float64Array(points.length + 1);
    const deleted = new Float64Array(points.length + 1);
    points.forEach((point, i) => {
      matches[i + 1] = matches[i] + point.count;
      created[i + 1] = created[i] + (point.created ?? 0);
      deleted[i + 1] = deleted[i] + (point.deleted ?? 0);
    });
    return { ticks, matches, created, deleted };
  }, [activity.points]);

  const bins = useMemo(() => {
    const binCount = showTicks ? end - start + 1 : count;
    const bins = [];
    let first = lowerBound(index.ticks, start);
    for (let bin = 0; bin < binCount; bin++) {
      const boundary =
        bin === binCount - 1
          ? end + 1
          : start +
            (showTicks ? bin + 1 : Math.ceil(((bin + 1) * span) / count));
      const after = lowerBound(index.ticks, boundary, first);
      if (after > first) {
        bins.push({
          bin,
          first,
          after,
          matches: index.matches[after] - index.matches[first],
          created: index.created[after] - index.created[first],
          deleted: index.deleted[after] - index.deleted[first],
        });
      }
      first = after;
    }
    return bins;
  }, [count, end, index, showTicks, span, start]);
  const peak = bins.reduce((max, bin) => Math.max(max, bin.matches), 1);
  const position = Math.max(0, Math.min(1, (tick - start) / span));

  return (
    <div
      className="activity-strip"
      ref={track}
      tabIndex={0}
      aria-label={`Timeline activity: ${activity.label}`}
      title={activity.label}
      onKeyDown={(event) => {
        if (
          event.altKey ||
          (event.key !== "ArrowLeft" && event.key !== "ArrowRight")
        )
          return;
        event.preventDefault();
        step(event.key === "ArrowRight");
      }}
    >
      {bins.map(({ bin, first, after, matches, created, deleted }) => {
        const firstTick = index.ticks[first];
        const lastTick = index.ticks[after - 1];
        const range =
          firstTick === lastTick
            ? `Tick ${numberFormat.format(firstTick)}`
            : `Ticks ${numberFormat.format(firstTick)}–${numberFormat.format(lastTick)}`;
        const lifecycle = [
          created ? `${numberFormat.format(created)} created` : "",
          deleted ? `${numberFormat.format(deleted)} deleted` : "",
        ].filter(Boolean);
        const details = [
          activity.label,
          range,
          `${numberFormat.format(matches)} ${matches === 1 ? "match" : "matches"}`,
          ...lifecycle,
        ];
        return (
          <button
            type="button"
            key={bin}
            className={
              showTicks ? "activity-bar activity-marker" : "activity-bar"
            }
            data-created={created > 0 || undefined}
            data-deleted={deleted > 0 || undefined}
            tabIndex={-1}
            title={details.join("\n")}
            aria-label={details.join(", ")}
            style={
              {
                left: showTicks
                  ? `clamp(0px, ${(bin / span) * 100}% - 4px, 100% - 8px)`
                  : `${(bin / count) * 100}%`,
                width: showTicks ? "8px" : `${100 / count}%`,
                "--activity-height": `${3 + Math.round(9 * Math.sqrt(matches / peak))}px`,
              } as React.CSSProperties
            }
            onClick={(event) => {
              const bounds = track.current!.getBoundingClientRect();
              const target =
                event.detail === 0
                  ? (firstTick + lastTick) / 2
                  : start +
                    ((event.clientX - bounds.left) / bounds.width) * span;
              const next = lowerBound(index.ticks, target, first, after);
              const left = index.ticks[Math.max(first, next - 1)];
              const right = index.ticks[Math.min(after - 1, next)];
              jump(
                Math.abs(right - target) < Math.abs(left - target)
                  ? right
                  : left,
              );
            }}
          />
        );
      })}
      <span
        className="activity-cursor"
        aria-hidden="true"
        style={{
          // Match the slider's 2px thumb inset at the ends of the range.
          left: `calc(${position * 100}% + ${1 - 2 * position}px)`,
        }}
      />
    </div>
  );
}
