import { useEffect, useRef } from "react";

interface Props {
  label: string;
  direction?: "ascending" | "descending";
  onSort?: () => void;
  onResize: (width?: number) => void;
}

export function SortLabel({
  label,
  direction,
  onSort,
}: Pick<Props, "label" | "direction"> & { onSort: () => void }) {
  return (
    <button
      type="button"
      className="sort-label"
      onClick={onSort}
      title={`Sort by ${label}`}
      aria-label={`Sort by ${label}, currently ${direction ?? "unsorted"}`}
    >
      {label}
      {direction && (
        <span aria-hidden="true">
          {direction === "ascending" ? " ↑" : " ↓"}
        </span>
      )}
    </button>
  );
}

export function FieldColumnHeader({
  label,
  direction,
  onSort,
  onResize,
}: Props) {
  const cell = useRef<HTMLSpanElement>(null);
  const stopDrag = useRef<() => void>(undefined);
  useEffect(() => () => stopDrag.current?.(), []);
  const width = () => cell.current!.getBoundingClientRect().width;
  const resize = (next: number) => onResize(Math.max(48, Math.round(next)));
  return (
    <span className="column-label" ref={cell}>
      {onSort ? (
        <SortLabel label={label} direction={direction} onSort={onSort} />
      ) : (
        <span className="sort-label">{label}</span>
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
