import { seekReplay } from "./seekReplay";
import { disposeReplayActivity } from "./replayActivity";
import initInspector, { ReplaySession } from "../generated/wasm/inspector";
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
    let cancelled = false;
    let parser: ReplaySession | undefined;

    const openReplay = async () => {
      try {
        setInitError(undefined);

        setLoadingStage("initializing web assembly");
        await initInspector();
        if (cancelled) {
          return;
        }

        setLoadingStage("loading file into memory");
        const fileBytes = new Uint8Array(await file.arrayBuffer());
        if (cancelled) {
          return;
        }

        setLoadingStage("constructing parser");
        parser = new ReplaySession(fileBytes);
        if (cancelled) {
          return;
        }

        setLoadingStage("seeking to first tick");
        seekReplay(parser, 0);
        if (cancelled) {
          return;
        }

        attachReplay(file, parser, parser.recordedTicks());
      } catch (error) {
        if (cancelled) {
          return;
        }
        setInitError(error);
      }
    };
    void openReplay();

    return () => {
      cancelled = true;
      disposeReplayActivity(file);
      parser?.free();
    };
  }, [file, attachReplay]);

  return { file, session, loadingStage, initError };
}
