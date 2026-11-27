import type { ActivityPoint, MessageLocation } from "../../replay/state";

export function decodeOccurrences(data: Uint32Array) {
  const locations: MessageLocation[] = [];
  const byTick = new Map<number, ActivityPoint>();
  for (let i = 0; i < data.length; i += 3) {
    const tick = data[i];
    const child = data[i + 2];
    locations.push({
      tick,
      command: data[i + 1],
      child: child === 0xffffffff ? null : child,
    });
    const point = byTick.get(tick);
    if (point) point.count++;
    else byTick.set(tick, { tick, count: 1 });
  }
  return { locations, points: [...byTick.values()] };
}

export function occurrenceNeighbors(
  locations: MessageLocation[],
  current: MessageLocation,
) {
  let low = 0;
  let high = locations.length;
  const compare = (candidate: MessageLocation) =>
    candidate.command - current.command ||
    (candidate.child ?? -1) - (current.child ?? -1);
  while (low < high) {
    const middle = low + Math.floor((high - low) / 2);
    if (compare(locations[middle]) < 0) low = middle + 1;
    else high = middle;
  }
  const exact = locations[low] && compare(locations[low]) === 0;
  return {
    previous: locations[low - 1],
    next: locations[low + (exact ? 1 : 0)],
  };
}
