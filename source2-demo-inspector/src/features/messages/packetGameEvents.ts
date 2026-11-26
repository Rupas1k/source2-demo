import type { Field } from "./MessageFields";
import { child } from "./packetStringTables";

import type { FiredEvent } from "../../replay/readGameEvents";

export type PacketGameEvent = Pick<FiredEvent, "id" | "name" | "fields"> & {
  definition?: true;
};

const keyTypes = [
  "none",
  "string",
  "float",
  "long",
  "short",
  "byte",
  "bool",
  "uint64",
  "int",
  "short",
];

export function gameEventDefinitions(fields: Field[]): PacketGameEvent[] {
  const descriptors =
    fields.find((field) => field.name === "descriptors")?.children ?? [];
  return descriptors.map((descriptor) => {
    const id = Number(child(descriptor, "eventid")?.value ?? 0);
    return {
      id,
      name: child(descriptor, "name")?.value ?? `Event ${id}`,
      definition: true,
      fields: (child(descriptor, "keys")?.children ?? []).map((key) => {
        const type = Number(child(key, "type")?.value ?? 0);
        return {
          name: child(key, "name")?.value ?? key.name,
          kind: keyTypes[type] ?? String(type),
          value: "",
        };
      }),
    };
  });
}
