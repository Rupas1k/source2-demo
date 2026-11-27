import { useRef, type CSSProperties, type ReactNode } from "react";
import { useInspectorPreferences } from "../app/preferences";

interface Props {
  records: ReactNode;
  details: ReactNode;
  recordTitle: string;
  detailTitle: string;
}

const minWidth = 12;
const maxWidth = 75;

export function RecordPanes({
  records,
  details,
  recordTitle,
  detailTitle,
}: Props) {
  const group = useRef<HTMLDivElement>(null);
  const recordsWidth = useInspectorPreferences((state) => state.recordsWidth);
  const setRecordsWidth = useInspectorPreferences(
    (state) => state.setRecordsWidth,
  );
  const drag = useRef<
    { pointer: number; width: number; left: number } | undefined
  >(undefined);
  const resize = (width: number) =>
    setRecordsWidth(Math.min(maxWidth, Math.max(minWidth, width)));

  return (
    <div
      className="panel-host record-split"
      ref={group}
      style={{ "--records-width": `${recordsWidth}%` } as CSSProperties}
    >
      <section className="record-pane">
        <h2 className="sr-only">{recordTitle}</h2>
        <div className="pane-content">{records}</div>
      </section>
      <div
        className="resize-grip"
        role="separator"
        tabIndex={0}
        aria-label="Resize record and property panes"
        aria-orientation="vertical"
        aria-valuemin={minWidth}
        aria-valuemax={maxWidth}
        aria-valuenow={Math.round(recordsWidth)}
        onPointerDown={(event) => {
          if (event.button !== 0 || !group.current) return;
          const { left, width } = group.current.getBoundingClientRect();
          if (!width) return;
          drag.current = { pointer: event.pointerId, width, left };
          event.currentTarget.setPointerCapture(event.pointerId);
          event.preventDefault();
        }}
        onPointerMove={(event) => {
          if (drag.current?.pointer === event.pointerId)
            resize(
              (100 * (event.clientX - drag.current.left)) / drag.current.width,
            );
        }}
        onLostPointerCapture={() => {
          drag.current = undefined;
        }}
        onKeyDown={(event) => {
          if (event.key === "ArrowLeft") resize(recordsWidth - 2);
          else if (event.key === "ArrowRight") resize(recordsWidth + 2);
          else if (event.key === "Home") resize(minWidth);
          else if (event.key === "End") resize(maxWidth);
          else return;
          event.preventDefault();
        }}
      />
      <section className="record-pane">
        <h2 className="sr-only">{detailTitle}</h2>
        <div className="pane-content">{details}</div>
      </section>
    </div>
  );
}
