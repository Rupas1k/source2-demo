import { useEffect, useRef } from "react";
import { ArrowDown, ArrowUp } from "../shared/components/Icons";

interface Props {
  label: string;
  direction?: "ascending" | "descending";
  alignEnd?: boolean;
  minimumWidth: number;
  onSort?: () => void;
  onResize: (width?: number) => void;
}

export function SortLabel({
  label,
  direction,
  alignEnd,
  onSort,
}: Pick<Props, "label" | "direction" | "alignEnd"> & { onSort: () => void }) {
  const Arrow = direction === "ascending" ? ArrowUp : ArrowDown;
  return (
    <button
      type="button"
      className="sort-label"
      data-align-end={alignEnd || undefined}
      onClick={onSort}
      title={`Sort by ${label}`}
      aria-label={`Sort by ${label}, currently ${direction ?? "unsorted"}`}
    >
      <span className="sort-label-text">{label}</span>
      {direction && <Arrow className="sort-arrow" size={12} />}
    </button>
  );
}

export function FieldColumnHeader({
  label,
  direction,
  alignEnd,
  onSort,
  onResize,
  minimumWidth,
}: Props) {
  const cell = useRef<HTMLSpanElement>(null);
  const stopDrag = useRef<() => void>(undefined);
  useEffect(() => () => stopDrag.current?.(), []);
  const width = () => cell.current!.getBoundingClientRect().width;
  const resize = (next: number) =>
    onResize(Math.max(minimumWidth, Math.round(next)));
  return (
    <span
      className="column-label"
      data-align-end={alignEnd || undefined}
      ref={cell}
    >
      {onSort ? (
        <SortLabel
          label={label}
          direction={direction}
          alignEnd={alignEnd}
          onSort={onSort}
        />
      ) : (
        <span className="sort-label">
          <span className="sort-label-text">{label}</span>
        </span>
      )}
      <button
        type="button"
        className="resize-column"
        aria-label={`Resize ${label} column`}
        title="Drag or use the left and right arrow keys to resize. Double-click or press Enter to reset."
        onPointerDown={(event) => {
          if (event.button !== 0) return;
          stopDrag.current?.();
          const { clientX, pointerId } = event;
          const startWidth = width();
          const move = (next: PointerEvent) => {
            if (next.pointerId === pointerId)
              resize(startWidth + next.clientX - clientX);
          };
          const finish = (next: PointerEvent) => {
            if (next.pointerId === pointerId) stopDrag.current?.();
          };
          stopDrag.current = () => {
            window.removeEventListener("pointermove", move);
            window.removeEventListener("pointerup", finish);
            window.removeEventListener("pointercancel", finish);
            stopDrag.current = undefined;
          };
          window.addEventListener("pointermove", move);
          window.addEventListener("pointerup", finish);
          window.addEventListener("pointercancel", finish);
          event.currentTarget.setPointerCapture(pointerId);
        }}
        onDoubleClick={() => onResize()}
        onKeyDown={(e) => {
          if (e.key === "ArrowLeft" || e.key === "ArrowRight") {
            e.preventDefault();
            resize(width() + (e.key === "ArrowLeft" ? -8 : 8));
          } else if (e.key === "Enter") onResize();
        }}
      />
    </span>
  );
}
