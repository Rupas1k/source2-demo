import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import { views } from "./views";

type ViewTab = (typeof views)[number]["id"];
type TableId =
  | "entity-fields"
  | "event-definitions"
  | "event-values"
  | "message-fields"
  | "string-tables"
  | "packet-string-tables";

export type NestedMessageBranchKind =
  "entities" | "stringTables" | "gameEvents";
type SharedMessageBranchKind = Exclude<NestedMessageBranchKind, "stringTables">;
export type MessageBranchKind =
  SharedMessageBranchKind | `full:${SharedMessageBranchKind}`;

interface InspectorPreferences {
  lastTab: ViewTab;
  recordsWidth: number;
  setRecordsWidth: (width: number) => void;
  messageExpansion: Partial<Record<MessageBranchKind, boolean>>;
  setMessageExpanded: (kind: MessageBranchKind, expanded: boolean) => void;
  widths: Partial<Record<TableId, Record<string, number>>>;
  setLastTab: (tab: ViewTab) => void;
  setColumnWidth: (table: TableId, column: string, width?: number) => void;
}

export const useInspectorPreferences = create<InspectorPreferences>()(
  persist(
    (set) => ({
      lastTab: views[0].id,
      recordsWidth: 23,
      setRecordsWidth: (recordsWidth) => set({ recordsWidth }),
      widths: {},
      messageExpansion: {},
      setMessageExpanded: (kind, expanded) =>
        set(({ messageExpansion }) => ({
          messageExpansion: { ...messageExpansion, [kind]: expanded },
        })),
      setLastTab: (lastTab) => set({ lastTab }),
      setColumnWidth: (table, column, width) =>
        set(({ widths }) => {
          const columns = { ...widths[table] };
          if (width === undefined) delete columns[column];
          else columns[column] = width;
          return { widths: { ...widths, [table]: columns } };
        }),
    }),
    {
      name: "source2-demo-inspector-preferences",
      storage: createJSONStorage(() => ({
        getItem: (key) => {
          try {
            return localStorage.getItem(key);
          } catch {
            return null;
          }
        },
        setItem: (key, value) => {
          try {
            localStorage.setItem(key, value);
          } catch {
            /* Browser storage is unavailable. */
          }
        },
        removeItem: (key) => {
          try {
            localStorage.removeItem(key);
          } catch {
            /* Browser storage is unavailable. */
          }
        },
      })),
      partialize: ({ lastTab, widths, messageExpansion, recordsWidth }) => ({
        lastTab,
        widths,
        messageExpansion,
        recordsWidth,
      }),
    },
  ),
);

export type { TableId };
