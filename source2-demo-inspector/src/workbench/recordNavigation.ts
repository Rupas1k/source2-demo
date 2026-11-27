import type { KeyboardEvent } from "react";

export function keepRecordListFocus(
  event: KeyboardEvent<HTMLDivElement>,
  viewport: HTMLDivElement | null,
) {
  // A focused row can be unmounted when the virtual list scrolls.
  if (event.target instanceof Node && viewport?.contains(event.target)) {
    event.currentTarget.focus({ preventScroll: true });
  }
}

export function nextRecordIndex(
  count: number,
  current: number,
  forward: boolean,
) {
  const start = current < 0 && !forward ? count : current;
  return Math.max(0, Math.min(count - 1, start + (forward ? 1 : -1)));
}
