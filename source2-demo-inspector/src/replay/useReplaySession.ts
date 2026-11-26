import { readReplayInput } from "./readReplayInput";
import { seekReplay } from "./seekReplay";
import { disposeReplayActivity } from "./replayActivity";
import initInspector, { type ReplaySession } from "../generated/wasm/inspector";
import { useEffect, useState } from "react";
import { useReplayStore } from "./state";

export function useReplaySession() {
  const file = useReplayStore((state) => state.replayFile);
  const session = useReplayStore((state) => state.replaySession);
  const attachReplay = useReplayStore((state) => state.attachReplay);

  const [loadingStage, setLoadingStage] = useState("Opening replay");
  const [initError, setInitError] = useState<unknown>();
  useEffect(() => {
    if (!file) return;
    const controller = new AbortController();
    const { signal } = controller;
    let parser: ReplaySession | undefined;

    const openReplay = async () => {
      try {
        setInitError(undefined);

        setLoadingStage("initializing web assembly");
        await initInspector();
        if (signal.aborted) {
          return;
        }

        setLoadingStage("loading file into memory");
        const input = await readReplayInput(file, signal);
        if (signal.aborted) {
          input.free();
          return;
        }

        setLoadingStage("constructing parser");
        parser = input.intoSession();
        if (signal.aborted) {
          return;
        }

        setLoadingStage("seeking to first tick");
        seekReplay(parser, 0);
        if (signal.aborted) {
          return;
        }

        attachReplay(file, parser, parser.recordedTicks());
      } catch (error) {
        if (signal.aborted) {
          return;
        }
        setInitError(error);
      }
    };
    void openReplay();

    return () => {
      controller.abort();
      disposeReplayActivity(file);
      parser?.free();
    };
  }, [file, attachReplay]);

  return { file, session, loadingStage, initError };
}
