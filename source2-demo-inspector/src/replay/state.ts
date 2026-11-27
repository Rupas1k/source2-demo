import { seekReplay } from "./seekReplay";
import type { StringTableRow } from "./readStringTable";
import { useInspectorPreferences } from "../app/preferences";
import { views } from "../app/views";
import { create } from "zustand";
import type { FieldSnapshot } from "./readFields";
import type { ReplaySession } from "../generated/wasm/inspector";

export type InspectionView =
  (typeof import("../app/views").views)[number]["id"] | "baselineEntities";

interface EntityComparison {
  tick: number;
  view: "entities" | "baselineEntities";
  index: number;
  fields: FieldSnapshot[];
}

interface StringTableComparison {
  tick: number;
  view: "stringTables";
  table: string;
  rows: StringTableRow[];
}

export type Comparison = EntityComparison | StringTableComparison;

export interface MessageLocation {
  tick: number;
  command: number;
  child: number | null;
}

export interface FieldSourceTarget {
  kind: "creation" | "update";
  handle: number;
  path: number[];
}

export type ActivityView = InspectionView;

export interface ActivityPoint {
  tick: number;
  count: number;
  created?: number;
  deleted?: number;
}

export interface TimelineActivity {
  label: string;
  points: ActivityPoint[];
  jump?: (tick: number) => void;
  occurrence?: {
    previous?: number;
    next?: number;
    step: (forward: boolean) => void;
  };
}

interface ReplayData {
  activity: Partial<Record<ActivityView, TimelineActivity>>;
  messageJump?: {
    location: MessageLocation;
    restore: () => void;
    field?: FieldSourceTarget;
  };
  replayFile?: File;
  replaySession?: ReplaySession;
  comparison?: Comparison;
  messageTicks?: Int32Array;
  selectedMessageTick?: number;
  selectedEventTick?: number;
  timelineError: string;
  currentTick: number;
  selectedEntity?: number;
  selectedBaseline?: number;
  selectedTable?: string;
  selectedEvent?: number;
  backHistory: Array<() => void>;
}

interface ReplayStore extends ReplayData {
  activeView: InspectionView;
  entityView: "entities" | "baselineEntities";
  openFile: (file: File | undefined) => void;
  attachReplay: (file: File, session: ReplaySession, ticks: Int32Array) => void;
  setSelectedMessageTick: (tick: number | undefined) => void;
  setSelectedEventTick: (tick: number | undefined) => void;
  setTimelineError: (error: string) => void;
  setActivity: (view: ActivityView, activity?: TimelineActivity) => void;
  setReplayPosition: (tick: number, comparison: Comparison | undefined) => void;
  setSelectedEntity: (index: number | undefined) => void;
  setSelectedBaseline: (index: number | undefined) => void;
  setSelectedTable: (name: string | undefined) => void;
  setSelectedEvent: (id: number | undefined) => void;
  setActiveView: (view: InspectionView) => void;
  openSourceMessage: (
    location: MessageLocation,
    restore: () => void,
    field?: FieldSourceTarget,
  ) => void;
  clearMessageJump: () => void;
  pushBack: (restore: () => void) => void;
  goBack: () => void;
}

function emptyReplay(replayFile?: File): ReplayData {
  return {
    replayFile,
    activity: {},
    messageJump: undefined,
    replaySession: undefined,
    comparison: undefined,
    messageTicks: undefined,
    selectedMessageTick: undefined,
    selectedEventTick: undefined,
    timelineError: "",
    currentTick: 0,
    selectedEntity: undefined,
    selectedBaseline: undefined,
    selectedTable: undefined,
    selectedEvent: undefined,
    backHistory: [],
  };
}

const { lastTab } = useInspectorPreferences.getState();

export const useReplayStore = create<ReplayStore>((set, get) => ({
  ...emptyReplay(),
  activeView: views.some(({ id }) => id === lastTab) ? lastTab : views[0].id,
  entityView: "entities",
  openFile: (file) => set(emptyReplay(file)),
  attachReplay: (file, replaySession, messageTicks) => {
    if (get().replayFile === file) {
      set({ replaySession, messageTicks, currentTick: replaySession.tick() });
    }
  },
  setSelectedMessageTick: (selectedMessageTick) => set({ selectedMessageTick }),
  setSelectedEventTick: (selectedEventTick) => set({ selectedEventTick }),
  setTimelineError: (timelineError) => set({ timelineError }),
  setActivity: (view, activity) =>
    set((state) => ({ activity: { ...state.activity, [view]: activity } })),
  setReplayPosition: (currentTick, comparison) =>
    set({ currentTick, comparison }),
  setSelectedEntity: (selectedEntity) => set({ selectedEntity }),
  setSelectedBaseline: (selectedBaseline) => set({ selectedBaseline }),
  setSelectedTable: (selectedTable) => set({ selectedTable }),
  setSelectedEvent: (selectedEvent) => set({ selectedEvent }),
  setActiveView: (activeView) =>
    set(
      activeView === "entities" || activeView === "baselineEntities"
        ? { activeView, entityView: activeView }
        : { activeView },
    ),
  openSourceMessage: (location, restore, field) =>
    set({ activeView: "messages", messageJump: { location, restore, field } }),
  clearMessageJump: () => set({ messageJump: undefined }),
  pushBack: (restore) => {
    const {
      replaySession,
      currentTick,
      selectedMessageTick,
      selectedEventTick,
      selectedEntity,
      selectedBaseline,
      selectedTable,
      selectedEvent,
      comparison,
    } = get();
    const restorePosition = () => {
      if (get().replaySession !== replaySession) return;
      try {
        const tick =
          replaySession && get().currentTick !== currentTick
            ? seekReplay(replaySession, currentTick)
            : currentTick;
        set({
          currentTick: tick,
          selectedMessageTick,
          selectedEventTick,
          selectedEntity,
          selectedBaseline,
          selectedTable,
          selectedEvent,
          comparison,
        });
        restore();
      } catch (error) {
        set({
          timelineError: error instanceof Error ? error.message : String(error),
        });
      }
    };
    set(({ backHistory }) => ({
      backHistory: [...backHistory, restorePosition],
    }));
  },
  goBack: () => {
    const { backHistory } = get();
    const restore = backHistory[backHistory.length - 1];
    if (restore) {
      set({ backHistory: backHistory.slice(0, -1) });
      restore();
    }
  },
}));

export const selectInspectedEntity = (state: ReplayStore) =>
  state.entityView === "baselineEntities"
    ? state.selectedBaseline
    : state.selectedEntity;
