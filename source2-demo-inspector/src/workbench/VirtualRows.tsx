import {
  useLayoutEffect,
  useMemo,
  type ReactElement,
  type RefObject,
} from "react";
import { Virtualizer, type VirtualizerHandle } from "virtua";

export type VirtualRange = { start: number; end: number };

const rowHeight = 26;
const buffer = 180;

export function VirtualRows({
  viewport,
  list,
  count,
  renderRow,
  onRangeChange,
}: {
  viewport: RefObject<HTMLDivElement | null>;
  list?: RefObject<VirtualizerHandle | null>;
  count: number;
  renderRow: (index: number) => ReactElement | null;
  onRangeChange?: (range: VirtualRange | undefined) => void;
}) {
  // Worker-backed lists need a length, without allocating one item per row.
  const data = useMemo<ArrayLike<unknown>>(() => ({ length: count }), [count]);

  function publishRange() {
    if (!onRangeChange) return;
    const handle = list?.current;
    if (!count || !handle) {
      onRangeChange(undefined);
      return;
    }
    const top = handle.scrollOffset;
    onRangeChange({
      start: handle.findItemIndex(Math.max(0, top - buffer)),
      end: handle.findItemIndex(
        top + Math.max(handle.viewportSize, rowHeight) + buffer,
      ),
    });
  }

  useLayoutEffect(publishRange);

  return (
    <Virtualizer
      ref={list}
      scrollRef={viewport}
      data={data}
      itemSize={rowHeight}
      bufferSize={buffer}
      as="ul"
      item="li"
      onScroll={onRangeChange ? publishRange : undefined}
      onResize={onRangeChange ? publishRange : undefined}
    >
      {(_, index) =>
        renderRow(index) ?? (
          <div aria-hidden="true" style={{ height: rowHeight }} />
        )
      }
    </Virtualizer>
  );
}
