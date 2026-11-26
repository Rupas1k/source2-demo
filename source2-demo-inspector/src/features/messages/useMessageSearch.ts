import { useEffect, useState } from "react";
import type { MessageInspector } from "../../generated/wasm/inspector";
import type { VirtualRange } from "../../workbench/VirtualRows";
import type { IndexedEntry, Query } from "./messageIndex";
import type { useMessageIndex } from "./useMessageIndex";

export function useMessageSearch(
  messageIndex: ReturnType<typeof useMessageIndex>,
  indexQuery: Query,
  isActive: boolean,
  inspector: MessageInspector | undefined,
) {
  const { term, wide } = indexQuery;
  const [search, setSearch] = useState<{
    query: Query;
    revision: number;
    rows: number;
    matches: number;
    activity: Uint32Array;
    error?: string;
  }>();
  const results =
    search?.query.term === term && search.query.wide === wide
      ? search
      : undefined;
  useEffect(() => {
    const call = messageIndex.current;
    if (!isActive || !inspector || !call || (!wide && !term)) return;
    let cancelled = false;
    const timer = setTimeout(
      () => {
        void call("query", indexQuery).then(
          (result) => {
            if (!cancelled) setSearch({ query: indexQuery, ...result });
          },
          (error) => {
            if (!cancelled)
              setSearch({
                query: indexQuery,
                revision: -1,
                rows: 0,
                matches: 0,
                activity: new Uint32Array(),
                error: String(error),
              });
          },
        );
      },
      search?.query.term === term ? 0 : 180,
    );
    return () => {
      cancelled = true;
      clearTimeout(timer);
    };
  }, [indexQuery, inspector, isActive, messageIndex, term, wide]);

  const [visible, setVisible] = useState<VirtualRange>();
  const { start, end } = visible ?? {};
  function updateVisible(next: VirtualRange | undefined) {
    setVisible((previous) =>
      previous?.start === next?.start && previous?.end === next?.end
        ? previous
        : next,
    );
  }
  const [range, setRange] = useState<{
    results: typeof results;
    start: number;
    entries: IndexedEntry[];
  }>();
  useEffect(() => {
    if (
      !wide ||
      !isActive ||
      !results ||
      start === undefined ||
      end === undefined
    )
      return;
    if (
      range?.results === results &&
      start >= range.start &&
      end < range.start + range.entries.length
    )
      return;
    const first = Math.max(0, start - 40);
    let cancelled = false;
    void messageIndex
      .current?.("range", {
        revision: results.revision,
        start: first,
        end: end + 41,
      })
      .then((entries) => {
        if (!cancelled && entries.length)
          setRange({ results, start: first, entries });
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  }, [end, isActive, messageIndex, range, results, start, wide]);
  function wideEntry(index: number) {
    return range && range.results === results
      ? range.entries[index - range.start]
      : undefined;
  }

  return { results, updateVisible, wideEntry, start, end };
}
