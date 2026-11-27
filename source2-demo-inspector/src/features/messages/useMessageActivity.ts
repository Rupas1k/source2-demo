import { useEffect, useRef, useState } from "react";
import type { MessageInspector } from "../../generated/wasm/inspector";
import {
  requestEntityOccurrences,
  requestStringTableOccurrences,
} from "../../replay/replayActivity";
import { useReplayStore, type MessageLocation } from "../../replay/state";
import { activityPoints, type Entry } from "./messageIndex";
import { decodeOccurrences, occurrenceNeighbors } from "./messageOccurrences";
import type { useMessageIndex } from "./useMessageIndex";

export function useMessageActivity({
  selected,
  entityHandle,
  entityName,
  entityFocus,
  eventId,
  eventName,
  tableName,
  file,
  inspector,
  isActive,
  messageIndex,
  term,
  results,
}: {
  selected?: Entry;
  entityHandle?: number;
  entityName?: string;
  entityFocus: boolean;
  eventId?: number;
  eventName?: string;
  tableName?: string;
  file?: File;
  inspector?: MessageInspector;
  isActive: boolean;
  messageIndex: ReturnType<typeof useMessageIndex>;
  term: string;
  results?: { activity: Uint32Array };
}) {
  const setActivity = useReplayStore((state) => state.setActivity);
  const activityJump = useRef<(target: number) => void>(undefined);
  const selectedOccurrenceJump =
    useRef<(target: MessageLocation) => void>(undefined);
  const activityId = selected?.id;
  const activityName = selected?.name;
  const activityIsPacket = selected != null && selected.child !== null;
  const occurrenceKey =
    eventId !== undefined
      ? `gameEvent:${eventId}:${eventName}`
      : tableName !== undefined
        ? `table:${tableName}`
        : entityFocus
          ? entityHandle === undefined
            ? undefined
            : `entity:${entityHandle}`
          : activityId === undefined
            ? undefined
            : `message:${activityIsPacket}:${activityId}`;
  const [occurrences, setOccurrences] = useState<{
    key: string;
    track: ReturnType<typeof decodeOccurrences>;
  }>();
  const selectedTrack =
    occurrences?.key === occurrenceKey ? occurrences?.track : undefined;
  useEffect(() => {
    if (!isActive || !inspector || !occurrenceKey) return;
    const result =
      eventId !== undefined && eventName !== undefined
        ? messageIndex.current?.("gameEventOccurrences", {
            id: eventId,
            name: eventName,
          })
        : tableName !== undefined && file
          ? requestStringTableOccurrences(file, tableName)
          : entityHandle !== undefined && file
            ? requestEntityOccurrences(file, entityHandle)
            : activityId === undefined
              ? undefined
              : messageIndex.current?.("typeOccurrences", {
                  id: activityId,
                  child: activityIsPacket,
                });
    let cancelled = false;
    void result
      ?.then((value) => {
        if (!cancelled)
          setOccurrences({
            key: occurrenceKey,
            track: decodeOccurrences(value),
          });
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  }, [
    activityId,
    activityIsPacket,
    entityHandle,
    eventId,
    eventName,
    file,
    inspector,
    isActive,
    messageIndex,
    occurrenceKey,
    tableName,
  ]);

  useEffect(() => {
    setActivity("messages");
    if (!isActive || !inspector) return;
    if (selected) {
      if (!selectedTrack) return;
      const { previous, next } = occurrenceNeighbors(
        selectedTrack.locations,
        selected,
      );
      setActivity("messages", {
        label:
          eventName ??
          tableName ??
          (entityFocus ? (entityName ?? "Entity updates") : activityName!),
        points: selectedTrack.points,
        jump: (target) => {
          const occurrence = selectedTrack.locations.find(
            (location) => location.tick === target,
          );
          if (occurrence) selectedOccurrenceJump.current?.(occurrence);
        },
        occurrence: {
          previous: previous?.tick,
          next: next?.tick,
          step: (forward) => {
            const target = forward ? next : previous;
            if (target) selectedOccurrenceJump.current?.(target);
          },
        },
      });
      return;
    }
    if (term) {
      if (results) {
        setActivity("messages", {
          label: `Messages matching “${term}”`,
          points: activityPoints(results.activity),
          jump: (target) => activityJump.current?.(target),
        });
      }
      return;
    }
    setActivity("messages", {
      label: "Demo commands",
      points: activityPoints(inspector.commandActivity()),
    });
  }, [
    activityName,
    entityFocus,
    entityName,
    eventName,
    inspector,
    isActive,
    results,
    selected,
    selectedTrack,
    setActivity,
    tableName,
    term,
  ]);

  return { activityJump, selectedOccurrenceJump };
}
