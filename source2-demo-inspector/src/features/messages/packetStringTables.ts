import type { Field } from "./MessageFields";

export interface PacketTable {
  id: number;
  name: string;
  index: number;
}

export function child(field: Field | undefined, name: string) {
  return field?.children?.find((entry) => entry.name === name);
}

export function snapshotTables(message: string, fields: Field[]) {
  const snapshot =
    message === "CDemoFullPacket"
      ? fields.find((field) => field.name === "string_table")?.children
      : message === "CDemoStringTables"
        ? fields
        : undefined;
  return snapshot?.find((field) => field.name === "tables")?.children ?? [];
}

export function packetTableRecords(tables: Field[]): PacketTable[] {
  return tables.map((table, index) => ({
    index,
    id: Number(child(table, "table_id")?.value ?? index),
    name: child(table, "table_name")?.value || table.name,
  }));
}
