import { useEffect, useRef } from "react";
import { workerClient } from "../../shared/workerClient";
import type {
  IndexMethod,
  MessageIndexApi,
  MessageIndexCalls,
} from "./messageIndex";

type IndexCall = <K extends IndexMethod>(
  method: K,
  input: MessageIndexCalls[K]["input"],
) => Promise<MessageIndexCalls[K]["output"]>;

export function useMessageIndex(file: File | undefined) {
  const call = useRef<IndexCall>(undefined);
  useEffect(() => {
    if (!file) return;
    let index: ReturnType<typeof workerClient<MessageIndexApi>> | undefined;
    call.current = (method, input) => {
      if (!index) {
        index = workerClient<MessageIndexApi>(
          new Worker(new URL("./messageIndex.worker.ts", import.meta.url), {
            type: "module",
          }),
        );
        void index.call((remote) => remote.open(file)).catch(() => {});
      }
      return index.call((remote) => {
        const run = remote[method] as (
          value: typeof input,
        ) => Promise<MessageIndexCalls[typeof method]["output"]>;
        return run(input);
      });
    };
    return () => {
      call.current = undefined;
      index?.close();
    };
  }, [file]);
  return call;
}
