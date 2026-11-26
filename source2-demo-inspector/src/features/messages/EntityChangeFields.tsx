import { useMemo } from "react";
import type { FieldSourceTarget } from "../../replay/state";
import {
  EntityFieldTable,
  useEntityFieldTable,
} from "../entities/EntityFieldTable";

interface FieldChange {
  path: number[];
  name: string;
  kind: string;
  decodedAs: string;
  before: string | null;
  after: string | null;
}

export interface EntityChange {
  index: number;
  handle: number;
  class: string;
  class_id: number;
  operation: "Created" | "Updated" | "Deleted";
  fields: FieldChange[];
}

export function findEntityChange(
  changes: EntityChange[],
  target: FieldSourceTarget,
) {
  const operation = target.kind === "creation" ? "Created" : "Updated";
  const path = target.path.join(".");
  return changes.reduce(
    (index, change, key) =>
      change.handle === target.handle &&
      change.operation === operation &&
      change.fields.some((field) => field.path.join(".") === path)
        ? key
        : index,
    -1,
  );
}

export function EntityChangeFields({
  change,
  error,
  targetPath,
  openEntity,
  openBaseline,
  openLinkedEntity,
}: {
  change?: EntityChange;
  error?: string;
  targetPath?: string;
  openEntity: (change: EntityChange) => void;
  openBaseline: (change: EntityChange) => void;
  openLinkedEntity: (handle: number) => void;
}) {
  const deleted = change?.operation === "Deleted";
  const fields = useMemo(
    () =>
      change?.fields.map((field) => ({
        path: Uint16Array.from(field.path),
        namedPath: field.name.split("."),
        encodedAs: field.kind,
        decodedAs: field.decodedAs,
        value: (deleted ? field.before : field.after) ?? "",
        write: { before: field.before ?? undefined, showTransition: !deleted },
      })),
    [change, deleted],
  );
  const table = useEntityFieldTable(fields, undefined, targetPath);
  return (
    <EntityFieldTable
      table={table}
      error={error}
      partial={!deleted}
      onFollow={(_, handle) => openLinkedEntity(handle)}
      actions={
        <>
          {change && !deleted && (
            <button onClick={() => openEntity(change)}>Open entity</button>
          )}
          {change?.operation === "Created" && (
            <button onClick={() => openBaseline(change)}>Open baseline</button>
          )}
        </>
      }
    />
  );
}
