import { useEffect, useRef, useState } from "react";
import type { Entry } from "./messageIndex";

export function messageKey(entry: Pick<Entry, "command" | "child">) {
  return entry.command + "/" + entry.child;
}

interface BranchResult<Row> {
  changes: Row[];
  error?: string;
}

export function useMessageBranch<Row>(
  request: (entry: Entry) => Promise<Row[]> | undefined,
) {
  const [results, setResults] = useState(new Map<string, BranchResult<Row>>());
  const pending = useRef(new Set<string>());
  const mounted = useRef(false);
  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
    };
  }, []);

  function load(entry: Entry) {
    const key = messageKey(entry);
    if (results.has(key) || pending.current.has(key)) return false;
    const response = request(entry);
    if (!response) return false;
    pending.current.add(key);
    void response.then(
      (changes) => save({ changes }),
      (error) => save({ changes: [], error: String(error) }),
    );
    return true;

    function save(result: BranchResult<Row>) {
      pending.current.delete(key);
      if (mounted.current)
        setResults((previous) => new Map(previous).set(key, result));
    }
  }

  return {
    results,
    load,
    loadNext: (entry: Entry) => !pending.current.size && load(entry),
  };
}
