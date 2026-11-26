import type {
  ReplaySession,
  EntityFieldRecord,
} from "../generated/wasm/inspector";
export type FieldSnapshot = Pick<
  EntityFieldRecord,
  "path" | "namedPath" | "value" | "encodedAs" | "decodedAs"
> & {
  write?: {
    before?: string;
    showTransition: boolean;
  };
};

/** Copy the selected tick's fields before advancing or releasing the Wasm session. */
export function readFields(
  session: ReplaySession | undefined,
  view: string,
  index: number | undefined,
): FieldSnapshot[] | undefined {
  if (!session || index === undefined) return;
  const records =
    view === "entities"
      ? session.listEntityFields(index)
      : view === "baselineEntities"
        ? session.listBaselineEntityFields(index)
        : undefined;
  return records?.map((record) => {
    const { path, namedPath, value, encodedAs, decodedAs } = record;
    record.free();
    return { path, namedPath, value, encodedAs, decodedAs };
  });
}
