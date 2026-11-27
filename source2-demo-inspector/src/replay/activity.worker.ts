import { readReplayInput } from "./readReplayInput";
import { expose, transfer } from "comlink";
import initInspector, {
  type ReplayActivityIndex,
} from "../generated/wasm/inspector";
import type { ActivityApi } from "./activityQuery";

let index: Promise<ReplayActivityIndex> | undefined;

const api: ActivityApi = {
  async read(file, query) {
    index ??= (async () => {
      await initInspector();
      return (await readReplayInput(file)).intoActivity();
    })();
    const activity = await index;
    const points =
      query.view === "entityOccurrences"
        ? activity.entityOccurrences(query.handle)
        : query.view === "stringTableOccurrences"
          ? activity.stringTableOccurrences(query.name)
          : query.view === "entities"
            ? activity.entities(query.handle)
            : query.view === "baselineEntities"
              ? activity.baselines(query.classId)
              : activity.stringTables(query.table, query.row);
    return transfer(points, [points.buffer as ArrayBuffer]);
  },
};
expose(api);
