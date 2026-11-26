import { expose, transfer } from "comlink";
import initInspector, {
  ReplayActivityIndex,
} from "../generated/wasm/inspector";
import type { ActivityApi } from "./activityQuery";

let index: Promise<ReplayActivityIndex> | undefined;

const api: ActivityApi = {
  async read(file, query) {
    index ??= Promise.all([initInspector(), file.arrayBuffer()]).then(
      ([, bytes]) => new ReplayActivityIndex(new Uint8Array(bytes)),
    );
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
