export function nextRecordIndex(
  count: number,
  current: number,
  forward: boolean,
) {
  const start = current < 0 && !forward ? count : current;
  return Math.max(0, Math.min(count - 1, start + (forward ? 1 : -1)));
}
