export type ActivityQuery =
  | { view: "entities"; handle?: number }
  | { view: "baselineEntities"; classId?: number }
  | { view: "stringTables"; table?: string; row?: number };

export type ReplayActivityQuery =
  | ActivityQuery
  | { view: "entityOccurrences"; handle: number }
  | { view: "stringTableOccurrences"; name: string };

export interface ActivityApi {
  read(file: File, query: ReplayActivityQuery): Promise<Uint32Array>;
}
