import type { FieldSnapshot } from "../../replay/readFields";

export type EntityFieldRow = {
  inner: FieldSnapshot;
  joinedNamedPath: string;
  depth: number;
  expandableKind?: "array" | "vector" | "vectorItem";
  collectionLength?: number;
};

export type EntityFieldGroup = {
  key: string;
  path: Uint16Array;
  namedPath: string[];
  kind: "array" | "vector" | "vectorItem";
  length: number;
  encodedAs: string;
};
