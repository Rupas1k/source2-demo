import type {
  ActivityApi,
  ActivityQuery,
  ReplayActivityQuery,
} from "./activityQuery";
import type { ActivityPoint } from "./state";
import { workerClient } from "../shared/workerClient";

const clients = new WeakMap<
  File,
  ReturnType<typeof workerClient<ActivityApi>>
>();

export async function requestReplayActivity(
  file: File,
  query: ActivityQuery,
): Promise<ActivityPoint[]> {
  const raw = await requestActivity(file, query);
  const points: ActivityPoint[] = [];
  for (let i = 0; i < raw.length; i += 4) {
    const [tick, count, created, deleted] = raw.subarray(i, i + 4);
    points.push({ tick, count, created, deleted });
  }
  return points;
}

export function requestEntityOccurrences(file: File, handle: number) {
  return requestActivity(file, { view: "entityOccurrences", handle });
}

export function requestStringTableOccurrences(file: File, name: string) {
  return requestActivity(file, { view: "stringTableOccurrences", name });
}

function requestActivity(file: File, query: ReplayActivityQuery) {
  let client = clients.get(file);
  if (!client) {
    client = workerClient<ActivityApi>(
      new Worker(new URL("./activity.worker.ts", import.meta.url), {
        type: "module",
      }),
    );
    clients.set(file, client);
  }
  return client.call((remote) => remote.read(file, query));
}

export function disposeReplayActivity(file: File) {
  clients.get(file)?.close();
  clients.delete(file);
}
