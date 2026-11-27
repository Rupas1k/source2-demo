import { useMessageActivity } from "./useMessageActivity";
import { readFields } from "../../replay/readFields";
import { seekReplay } from "../../replay/seekReplay";
import { useMessageSearch } from "./useMessageSearch";
import { readMessageData as read } from "./readMessageData";
import { sortRows, useTableSort } from "../../workbench/tableSort";
import { useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import type { MessageInspector } from "../../generated/wasm/inspector";
import { useReplayStore, type FieldSourceTarget } from "../../replay/state";
import type { VirtualizerHandle } from "virtua";
import { type Field } from "./MessageFields";
import { findEntityChange } from "./EntityChangeFields";
import { messageKey, useMessageBranch } from "./useMessageBranch";
import { snapshotTables } from "./packetStringTables";

import { isFullPacket, type Entry, type Query } from "./messageIndex";
import { useMessageIndex } from "./useMessageIndex";

import { branchKind, type NestedMessageBranchKind } from "./messageBranches";
function matchesMessage(entry: Entry, term: string) {
  return entry.name.toLowerCase().includes(term) || String(entry.id) === term;
}

export function useMessageNavigation({
  inspector,
  tick,
  error,
}: {
  inspector?: MessageInspector;
  tick: number;
  error: string;
}) {
  const isActive = useReplayStore((state) => state.activeView === "messages");
  const sourceJump = useReplayStore((state) => state.messageJump);
  const clearSourceJump = useReplayStore((state) => state.clearMessageJump);
  const data = useMemo(
    () =>
      read<Entry[]>(
        () => (inspector ? JSON.parse(inspector.entries(tick)) : []),
        [],
      ),
    [inspector, tick],
  );
  const selectTick = useReplayStore((state) => state.setSelectedMessageTick);
  const pushBack = useReplayStore((state) => state.pushBack);
  const session = useReplayStore((state) => state.replaySession);
  const [picked, pick] = useState<Entry>();
  const selected = picked?.tick === tick ? picked : undefined;
  const [manualPacketExpansion, setPacketExpansion] = useState<
    ReadonlyMap<number, boolean>
  >(() => new Map());
  const [branchExpansion, setBranchExpansion] = useState<
    ReadonlyMap<string, boolean>
  >(() => new Map());
  const [revealedParent, setRevealedParent] = useState<number>();
  const revealedPacket =
    selected?.command === revealedParent ? revealedParent : undefined;
  const [reveal, setReveal] = useState<Entry>();
  const [query, setQuery] = useState("");
  const [fieldQuery, setFieldQuery] = useState("");
  const [wide, setWide] = useState(false);
  const file = useReplayStore((state) => state.replayFile);
  const messageIndex = useMessageIndex(file);
  const [linkError, setLinkError] = useState("");
  const entityChanges = useMessageBranch((entry) =>
    entry.child === null
      ? undefined
      : messageIndex.current?.("entityChanges", {
          command: entry.command,
          child: entry.child,
        }),
  );
  const stringTables = useMessageBranch((entry) =>
    messageIndex.current?.("stringTables", entry),
  );
  const gameEvents = useMessageBranch((entry) =>
    messageIndex.current?.("gameEvents", entry),
  );
  const [revealedBranchKey, setRevealedBranchKey] = useState<string>();
  const [eventSelection, selectPacketEvent] = useState<{
    key: string;
    id: number;
    name: string;
    definition?: true;
    reveal?: boolean;
  }>();
  const [entitySelection, selectChangedEntity] = useState<{
    key: string;
    index?: number;
    target?: FieldSourceTarget;
    handle?: number;
  }>();
  const [tableSelection, selectPacketTable] = useState<{
    key: string;
    name: string;
    reveal?: boolean;
  }>();
  const selectedKey = selected && messageKey(selected);
  const eventFocus =
    eventSelection?.key === selectedKey ? eventSelection : undefined;
  const eventId = eventFocus?.definition ? undefined : eventFocus?.id;
  const eventName = eventFocus?.name;
  const tableFocus =
    tableSelection?.key === selectedKey ? tableSelection : undefined;
  const tableName = tableFocus?.name;
  const entityFocus =
    entitySelection?.key === selectedKey ? entitySelection : undefined;
  const selectedBranch = selectedKey
    ? entityChanges.results.get(selectedKey)
    : undefined;
  const changedEntityIndex =
    entityFocus?.target && selectedBranch
      ? findEntityChange(selectedBranch.changes, entityFocus.target)
      : entityFocus?.handle !== undefined && selectedBranch
        ? selectedBranch.changes.findIndex(
            ({ handle }) => handle === entityFocus.handle,
          )
        : (entityFocus?.index ?? -1);
  const changedEntity = selectedBranch?.changes[changedEntityIndex];

  const indexSort = useTableSort<"name" | "id" | "tick">();
  const { sort: messageSort } = indexSort;
  const term = query.trim().toLowerCase();
  const [searchClosedPackets, setSearchClosedPackets] =
    useState<ReadonlySet<number>>();
  useEffect(() => setSearchClosedPackets(undefined), [term]);
  const packetExpansion = useMemo(() => {
    if (!term) return manualPacketExpansion;
    const effective = new Map<number, boolean>();
    for (const [command, expanded] of manualPacketExpansion) {
      if (expanded || searchClosedPackets?.has(command))
        effective.set(command, expanded);
    }
    return effective;
  }, [term, manualPacketExpansion, searchClosedPackets]);

  const indexQuery = useMemo<Query>(
    () => ({
      term,
      wide,
      sort: messageSort,
      packetExpansion,
      revealedPacket,
    }),
    [term, wide, messageSort, packetExpansion, revealedPacket],
  );
  const { results, updateVisible, wideEntry, start, end } = useMessageSearch(
    messageIndex,
    indexQuery,
    isActive,
    inspector,
  );

  const packets = useMemo(
    () =>
      new Set(
        data.value
          .filter((entry) => entry.child !== null)
          .map((entry) => entry.command),
      ),
    [data],
  );
  const entityHandle =
    changedEntity?.handle ?? entityFocus?.target?.handle ?? entityFocus?.handle;
  const entityName = changedEntity?.class;
  const { activityJump, selectedOccurrenceJump } = useMessageActivity({
    selected,
    entityHandle,
    entityName,
    entityFocus: !!entityFocus,
    eventId,
    eventName,
    tableName,
    file,
    inspector,
    isActive,
    messageIndex,
    term,
    results,
  });

  const matchedCommands = useMemo(() => {
    const parents = new Set<number>();
    const children = new Set<number>();
    const hits = new Set<Entry>();
    if (term) {
      for (const entry of data.value) {
        if (!matchesMessage(entry, term)) continue;
        hits.add(entry);
        (entry.child === null ? parents : children).add(entry.command);
      }
    }
    return { parents, children, hits };
  }, [data, term]);
  const expandablePackets = useMemo(
    () =>
      data.value.filter(
        (entry) =>
          entry.child === null &&
          (packets.has(entry.command) || isFullPacket(entry)),
      ),
    [data.value, packets],
  );
  const defaultPacket =
    !wide && !term && expandablePackets.length === 1
      ? expandablePackets[0].command
      : undefined;
  const openPackets = useMemo(
    () =>
      new Set(
        expandablePackets
          .filter(
            ({ command }) =>
              command === revealedPacket ||
              (packetExpansion.get(command) ??
                (command === defaultPacket ||
                  matchedCommands.children.has(command))),
          )
          .map(({ command }) => command),
      ),
    [
      expandablePackets,
      revealedPacket,
      packetExpansion,
      defaultPacket,
      matchedCommands.children,
    ],
  );

  const localRows = useMemo(() => {
    if (wide) return [];
    const sort = messageSort?.column === "tick" ? undefined : messageSort;
    const order = (entries: Entry[]) =>
      sortRows(entries, sort, (entry, column) => entry[column]);
    const rows = data.value.filter((entry) => {
      const parentMatches = !term || matchedCommands.parents.has(entry.command);
      if (entry.child === null)
        return (
          parentMatches ||
          matchedCommands.children.has(entry.command) ||
          entry.command === revealedPacket
        );
      return (
        openPackets.has(entry.command) &&
        (parentMatches ||
          matchedCommands.hits.has(entry) ||
          (entry.command === revealedPacket && entry.child === selected?.child))
      );
    });
    if (!sort) return rows;
    const groups = new Map<number, { parent: Entry; children: Entry[] }>();
    for (const entry of rows) {
      if (entry.child === null)
        groups.set(entry.command, { parent: entry, children: [] });
      else groups.get(entry.command)?.children.push(entry);
    }
    return order(Array.from(groups.values(), (group) => group.parent)).flatMap(
      (parent) => [parent, ...order(groups.get(parent.command)!.children)],
    );
  }, [
    data,
    term,
    matchedCommands,
    openPackets,
    wide,
    messageSort,
    revealedPacket,
    selected?.child,
  ]);

  const details = useMemo(
    () =>
      read<Field[]>(
        () =>
          inspector && selected
            ? JSON.parse(
                inspector.inspect(
                  selected.command,
                  selected.child ?? undefined,
                ),
              )
            : [],
        [],
      ),
    [inspector, selected],
  );
  const selectedHasNoEntityEntries =
    Number(
      details.value.find((field) => field.name === "updated_entries")?.value,
    ) === 0;
  const tableBranch = selectedKey
    ? stringTables.results.get(selectedKey)
    : undefined;
  const selectedTable = tableBranch?.changes.find(
    (table) => table.name === tableName,
  );
  const tableDetails = useMemo(() => {
    if (!tableFocus || !selected || !selectedTable || !inspector)
      return undefined;
    if (selected.child === null)
      return {
        value: snapshotTables(selected.name, details.value)[
          selectedTable.index
        ],
        error: "",
      };
    const child = selected.child;
    return read<Field | undefined>(
      () =>
        (
          JSON.parse(
            inspector.stringTableChanges(selected.command, child),
          ) as Field[]
        )[selectedTable.index],
      undefined,
    );
  }, [details, inspector, selected, selectedTable, tableFocus]);
  const viewport = useRef<HTMLDivElement>(null);
  const restoreScroll = useRef<number>(undefined);
  const branchRow = useRef<HTMLButtonElement>(null);
  const revealedBranch = useRef<
    typeof entityFocus | typeof tableFocus | typeof eventFocus
  >(undefined);

  const list = useRef<VirtualizerHandle>(null);
  const count = wide ? (results?.rows ?? 0) : localRows.length;
  useLayoutEffect(() => {
    if (restoreScroll.current === undefined || (wide && !results) || !isActive)
      return;
    viewport.current?.scrollTo({ top: restoreScroll.current });
    restoreScroll.current = undefined;
  });
  useLayoutEffect(() => {
    if (!reveal || reveal.tick !== tick || (wide && !results)) return;
    let cancelled = false;
    function show(position: number) {
      if (cancelled || position < 0) return;
      list.current?.scrollToIndex(position, {
        align: entityFocus || tableFocus || eventFocus ? "start" : "nearest",
      });
      setReveal(undefined);
    }
    if (wide) {
      void messageIndex
        .current?.("position", { ...reveal, revision: results!.revision })
        .then(show)
        .catch(() => {});
    } else {
      show(
        localRows.findIndex(
          (entry) =>
            entry.command === reveal.command && entry.child === reveal.child,
        ),
      );
    }
    return () => {
      cancelled = true;
    };
  }, [
    entityFocus,
    eventFocus,
    list,
    localRows,
    messageIndex,
    results,
    reveal,
    tableFocus,
    tick,
    wide,
  ]);

  useLayoutEffect(() => {
    const focus = eventFocus?.reveal
      ? eventFocus
      : tableFocus?.reveal
        ? tableFocus
        : changedEntity &&
            (entityFocus?.target || entityFocus?.handle !== undefined)
          ? entityFocus
          : undefined;
    if (!focus || reveal || revealedBranch.current === focus) return;
    let frame: number;
    let stable = false;
    let attempts = 0;
    function revealRow() {
      const element = branchRow.current;
      const container = viewport.current;
      if (!element || !container || attempts++ === 8) return;
      const row = element.getBoundingClientRect();
      const bounds = container.getBoundingClientRect();
      const delta =
        row.top < bounds.top
          ? row.top - bounds.top
          : Math.max(0, row.bottom - bounds.bottom);
      if (Math.abs(delta) > 1) {
        stable = false;
        container.scrollTop += delta;
      } else if (stable) {
        revealedBranch.current = focus;
        return;
      } else {
        stable = true;
      }
      frame = requestAnimationFrame(revealRow);
    }
    frame = requestAnimationFrame(revealRow);
    return () => cancelAnimationFrame(frame);
  });

  function canExpandBranch(entry: Entry) {
    const kind = branchKind(entry);
    if (!kind) return false;
    const key = messageKey(entry);
    const branch =
      kind === "entities"
        ? entityChanges.results.get(key)
        : kind === "stringTables"
          ? stringTables.results.get(key)
          : gameEvents.results.get(key);
    if (branch) return branch.changes.length > 0 || !!branch.error;
    if (key !== selectedKey) return true;
    if (kind === "entities") return !selectedHasNoEntityEntries;
    if (kind === "stringTables" && entry.child === null)
      return snapshotTables(entry.name, details.value).length > 0;
    return true;
  }

  const expandableMessages =
    wide || term
      ? []
      : localRows.filter(
          (entry) => entry.child !== null && canExpandBranch(entry),
        );
  const defaultEntityBranch =
    expandableMessages.length === 1 &&
    branchKind(expandableMessages[0]) === "entities"
      ? messageKey(expandableMessages[0])
      : undefined;

  function isBranchOpen(entry: Entry) {
    const key = messageKey(entry);
    if (isFullPacket(entry)) {
      const snapshotVisible =
        !term ||
        matchesMessage(entry, term) ||
        (selectedKey === key && revealedBranchKey === key);
      return snapshotVisible && isPacketOpen(entry);
    }
    return (
      (branchExpansion.get(key) ?? key === defaultEntityBranch) ||
      (selectedKey === key && revealedBranchKey === key)
    );
  }

  useEffect(() => {
    if (!isActive) return;
    if (start === undefined || end === undefined) return;
    for (let index = start; index <= end; index++) {
      const entry = wide ? wideEntry(index) : localRows[index];
      if (!entry) continue;
      const kind = branchKind(entry);
      if (!kind || !isBranchOpen(entry)) continue;
      if (kind === "entities") {
        if (messageKey(entry) !== selectedKey || !selectedHasNoEntityEntries)
          entityChanges.loadNext(entry);
      } else if (kind === "stringTables") {
        stringTables.loadNext(entry);
      } else {
        gameEvents.loadNext(entry);
      }
    }
  });

  const latestIndex = useRef({ results, tick });
  useLayoutEffect(() => {
    latestIndex.current = { results, tick };
  }, [results, tick]);

  async function navigateWide(forward: boolean) {
    const call = messageIndex.current;
    if (!call || !results?.rows) return;
    try {
      const current = selected
        ? await call("position", { ...selected, revision: results.revision })
        : -1;
      const from = current < 0 && !forward ? results.rows : current;
      const next = Math.max(
        0,
        Math.min(results.rows - 1, from + (forward ? 1 : -1)),
      );
      const entry =
        wideEntry(next) ??
        (
          await call("range", {
            revision: results.revision,
            start: next,
            end: next + 1,
          })
        )[0];
      if (
        !entry ||
        latestIndex.current.results !== results ||
        latestIndex.current.tick !== tick
      )
        return;
      selectMessage(entry);
      list.current?.scrollToIndex(next, { align: "nearest" });
    } catch {
      // Ignore requests whose replay was closed.
    }
  }

  function jumpToMessage(entry: Entry, keepFilter = false) {
    selectChangedEntity(undefined);
    selectPacketTable(undefined);
    selectPacketEvent(undefined);
    pick(entry);
    setLinkError("");
    if (!keepFilter) {
      setQuery("");
      setWide(false);
    }
    selectTick(entry.tick);
    setRevealedParent(entry.child === null ? undefined : entry.command);
    setRevealedBranchKey(undefined);
    setReveal(entry);
  }

  useLayoutEffect(() => {
    selectedOccurrenceJump.current = (target) => {
      if (!inspector) return;
      const entries =
        target.tick === tick
          ? data.value
          : read<Entry[]>(() => JSON.parse(inspector.entries(target.tick)), [])
              .value;
      const entry = entries.find(
        ({ command, child }) =>
          command === target.command && child === target.child,
      );
      if (!entry) {
        if (target.tick !== tick)
          read(() => inspector.entries(tick), undefined);
        setLinkError("The selected occurrence is unavailable.");
        return;
      }
      jumpToMessage(entry, true);
      const matchesContext = entries.some(
        (candidate) =>
          candidate.command === entry.command &&
          (entry.child === null || (!wide && candidate.child === null)) &&
          matchesMessage(candidate, term),
      );
      if (term && !matchesMessage(entry, term) && !matchesContext) setQuery("");
      if (eventFocus) {
        revealBranch(entry, "gameEvents");
        selectPacketEvent({
          ...eventFocus,
          key: messageKey(entry),
          reveal: true,
        });
      } else if (tableName !== undefined) {
        revealBranch(entry, "stringTables");
        selectPacketTable({
          key: messageKey(entry),
          name: tableName,
          reveal: true,
        });
      } else if (entityHandle !== undefined) {
        revealBranch(entry, "entities");
        selectChangedEntity({ key: messageKey(entry), handle: entityHandle });
      }
    };
    activityJump.current = (target) => {
      if (!results || !term) return;
      void messageIndex
        .current?.("occurrence", { revision: results.revision, tick: target })
        .then((occurrence) => {
          if (
            !occurrence ||
            latestIndex.current.results !== results ||
            latestIndex.current.tick !== tick
          )
            return;
          rememberMessage();
          jumpToMessage(occurrence, true);
        })
        .catch(() => {});
    };
    return () => {
      activityJump.current = undefined;
      selectedOccurrenceJump.current = undefined;
    };
  });

  function captureMessage(restore?: () => void) {
    const scroll = viewport.current?.scrollTop ?? 0;
    return () => {
      selectTick(tick);
      pick(picked);
      setRevealedParent(revealedParent);
      setQuery(query);
      setWide(wide);
      setFieldQuery(fieldQuery);
      setRevealedBranchKey(revealedBranchKey);
      selectPacketEvent(eventSelection);
      selectChangedEntity(entitySelection);
      setReveal(undefined);
      selectPacketTable(tableSelection);
      setLinkError(linkError);
      restore?.();
      restoreScroll.current = scroll;
    };
  }

  function rememberMessage(restore?: () => void) {
    pushBack(captureMessage(restore));
  }

  useEffect(() => {
    if (!sourceJump || !inspector) return;
    const { location, restore, field } = sourceJump;
    const match = read<Entry | undefined>(
      () =>
        (JSON.parse(inspector.entries(location.tick)) as Entry[]).find(
          (entry) =>
            entry.command === location.command &&
            entry.child === location.child,
        ),
      undefined,
    );
    // Restore the packet cache if the jump cannot select an occurrence.
    if (!match.value) {
      read(() => inspector.entries(tick), undefined);
      setLinkError(match.error || "The source message is unavailable.");
      pushBack(restore);
    } else {
      rememberMessage(restore);
      jumpToMessage(match.value);
      if (field) {
        revealBranch(match.value, "entities");
        selectChangedEntity({ key: messageKey(match.value), target: field });
      }
    }
    clearSourceJump();
  }, [sourceJump, inspector]);
  function isPacketOpen(entry: Entry) {
    if (!wide) return openPackets.has(entry.command);
    return (
      entry.command === revealedPacket ||
      (packetExpansion.get(entry.command) ??
        ("revealChildren" in entry && entry.revealChildren === true))
    );
  }

  function hasChildren(entry: Entry) {
    return wide
      ? "hasChildren" in entry && entry.hasChildren === true
      : packets.has(entry.command);
  }

  function togglePacket(entry: Entry) {
    const { command } = entry;
    const closing = isPacketOpen(entry);
    if (revealedParent === command) setRevealedParent(undefined);
    setPacketExpansion((previous) => new Map(previous).set(command, !closing));
    if (term && closing)
      setSearchClosedPackets((previous) => new Set(previous).add(command));
    if (closing && isFullPacket(entry) && selected?.command === command)
      selectPacketTable(undefined);
  }
  function revealBranch(entry: Entry, kind: NestedMessageBranchKind) {
    setRevealedBranchKey(messageKey(entry));
    if (isFullPacket(entry)) setRevealedParent(entry.command);
    if (kind === "entities") entityChanges.load(entry);
    else if (kind === "stringTables") stringTables.load(entry);
    else gameEvents.load(entry);
  }

  function toggleBranch(entry: Entry, kind: NestedMessageBranchKind) {
    const key = messageKey(entry);
    const open = !isBranchOpen(entry);
    setBranchExpansion((previous) => new Map(previous).set(key, open));
    if (open) {
      if (kind === "entities") entityChanges.load(entry);
      else if (kind === "stringTables") stringTables.load(entry);
      else gameEvents.load(entry);
    } else if (selectedKey === key) {
      setRevealedBranchKey(undefined);
      if (kind === "entities") selectChangedEntity(undefined);
      else if (kind === "stringTables") selectPacketTable(undefined);
      else selectPacketEvent(undefined);
    }
  }

  function openEntityView(value: number, baseline: boolean) {
    if (!session) return;
    const store = useReplayStore.getState();
    try {
      const actual = seekReplay(session, tick);
      const entity = baseline ? value : session.entityIndexForHandle(value);
      if (entity === undefined)
        throw new Error("This entity is absent at the inspected tick.");
      if (baseline && !readFields(session, "baselineEntities", entity)) {
        throw new Error("This baseline is unavailable at the inspected tick.");
      }
      rememberMessage(() => store.setActiveView("messages"));
      if (baseline) store.setSelectedBaseline(entity);
      else store.setSelectedEntity(entity);
      store.setReplayPosition(actual, undefined);
      store.setActiveView(baseline ? "baselineEntities" : "entities");
    } catch (error) {
      try {
        seekReplay(session, store.currentTick);
        setLinkError(String(error));
      } catch (restoreError) {
        store.setReplayPosition(session.tick(), store.comparison);
        setLinkError(String(restoreError));
      }
    }
  }

  function selectMessage(entry: Entry) {
    selectChangedEntity(undefined);
    selectPacketTable(undefined);
    selectPacketEvent(undefined);
    pick(entry);
    setLinkError("");
    if (wide) selectTick(entry.tick);
  }

  function activateMessage(entry: Entry) {
    const isSelected =
      selected?.command === entry.command && selected.child === entry.child;
    if (isSelected && !entityFocus && !tableFocus && !eventFocus) {
      pick(undefined);
      setLinkError("");
      return;
    }
    selectMessage(entry);
  }

  const problem =
    error ||
    data.error ||
    details.error ||
    (wide && results?.error) ||
    linkError;
  return {
    listProps: {
      wide,
      results,
      localRows,
      selected,
      activateMessage,
      navigateWide,
      selectMessage,
      list,
      query,
      term,
      matchedCommands,
      setQuery,
      setWide,
      viewport,
      indexSort,
      count,
      updateVisible,
      wideEntry,
      hasChildren,
      entityChanges,
      stringTables,
      gameEvents,
      isBranchOpen,
      canExpandBranch,
      selectedKey,
      details,
      entityFocus,
      tableFocus,
      eventFocus,
      isPacketOpen,
      togglePacket,
      toggleBranch,
      inspector,
      branchRow,
      changedEntityIndex,
      selectChangedEntity,
      selectPacketTable,
      selectPacketEvent,
    },
    detailsProps: {
      fieldQuery,
      setFieldQuery,
      eventFocus,
      gameEvents,
      linkError,
      tableFocus,
      inspector,
      tableDetails,
      tableBranch,
      entityFocus,
      changedEntityIndex,
      changedEntity,
      selectedBranch,
      openEntityView,
      problem,
      selected,
      details,
    },
  };
}
