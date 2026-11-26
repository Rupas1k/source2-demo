import { ArrowLeft, FolderOpen, X } from "../shared/components/Icons";
import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
} from "react";

import DemoTimeline from "../workbench/DemoTimeline";
import DemoScreen from "../workbench/DemoScreen";
import { useReplayStore } from "../replay/state";
import { useReplaySession } from "../replay/useReplaySession";
import { useInspectorPreferences } from "./preferences";
import { views } from "./views";

export default function App() {
  const { file, session: parser, loadingStage, initError } = useReplaySession();
  const setFile = useReplayStore((state) => state.openFile);
  const view = useReplayStore((state) => state.activeView);
  const setLastTab = useInspectorPreferences((state) => state.setLastTab);
  const setView = useReplayStore((state) => state.setActiveView);
  const timelineError = useReplayStore((state) => state.timelineError);
  const setTimelineError = useReplayStore((state) => state.setTimelineError);

  const canGoBack = useReplayStore((state) => state.backHistory.length > 0);
  const goBack = useReplayStore((state) => state.goBack);
  const [error, setError] = useState("");
  const [dismissedInitError, setDismissedInitError] = useState<{
    file: File;
    error: unknown;
  }>();
  const fileVersion = useRef(0);
  const fileInput = useRef<HTMLInputElement>(null);
  const dragDepth = useRef(0);
  const [isDragActive, setDragActive] = useState(false);
  const open = useCallback(() => fileInput.current?.click(), []);
  function openReplay(files: FileList) {
    if (files.length !== 1 || !files[0].name.toLowerCase().endsWith(".dem")) {
      setError("Choose one Source 2 replay with a .dem extension.");
      return;
    }
    setError("");
    fileVersion.current++;
    setFile(files[0]);
  }

  useLayoutEffect(() => {
    const media = matchMedia("(prefers-color-scheme: dark)");
    const update = () => {
      const dark = media.matches;
      document.documentElement.className = dark ? "dark" : "light";
    };
    update();
    media.addEventListener("change", update);
    return () => media.removeEventListener("change", update);
  }, []);

  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (event.defaultPrevented) return;
      const escapeBack =
        event.key === "Escape" &&
        !event.altKey &&
        !event.ctrlKey &&
        !event.metaKey &&
        !event.shiftKey;
      if (canGoBack && escapeBack) {
        event.preventDefault();
        if (!event.repeat) goBack();
        return;
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "o") {
        event.preventDefault();
        open();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [open, canGoBack, goBack]);

  const tab = view === "baselineEntities" ? "entities" : view;
  const activeTab = useRef<HTMLButtonElement>(null);
  useLayoutEffect(() => {
    activeTab.current?.scrollIntoView({ block: "nearest", inline: "nearest" });
  }, [tab]);

  const replayError =
    file &&
    initError &&
    !(
      dismissedInitError?.file === file &&
      dismissedInitError.error === initError
    )
      ? `Unable to open replay. ${initError instanceof Error ? initError.message : String(initError)}`
      : "";
  const headerError = error || replayError || timelineError;
  function dismissError() {
    if (!headerError) return;
    if (error) setError("");
    if (timelineError) setTimelineError("");
    if (file && replayError) setDismissedInitError({ file, error: initError });
  }
  const pageTitle = views.find((item) => item.id === tab)?.label;
  return (
    <div
      className="simple-desk"
      onDragEnter={(event) => {
        if (!event.dataTransfer.types.includes("Files")) return;
        event.preventDefault();
        dragDepth.current++;
        setDragActive(true);
      }}
      onDragOver={(event) => {
        if (!event.dataTransfer.types.includes("Files")) return;
        event.preventDefault();
        event.dataTransfer.dropEffect = "copy";
      }}
      onDragLeave={() => {
        dragDepth.current = Math.max(0, dragDepth.current - 1);
        if (!dragDepth.current) setDragActive(false);
      }}
      onDrop={(event) => {
        event.preventDefault();
        dragDepth.current = 0;
        setDragActive(false);
        if (event.dataTransfer.files.length)
          openReplay(event.dataTransfer.files);
      }}
      onClickCapture={dismissError}
      onKeyDownCapture={(event) => {
        if (!["Alt", "Control", "Meta", "Shift"].includes(event.key))
          dismissError();
      }}
      onWheelCapture={dismissError}
    >
      <input
        ref={fileInput}
        type="file"
        accept=".dem"
        aria-label="Open replay file"
        hidden
        onChange={(event) => {
          if (event.currentTarget.files?.length)
            openReplay(event.currentTarget.files);
          event.currentTarget.value = "";
        }}
      />
      <header className="app-header">
        <div className="toolbar simple-bar">
          <button
            type="button"
            className="header-icon-button"
            aria-label="Back"
            title="Back (Esc)"
            aria-keyshortcuts="Escape"
            disabled={!canGoBack}
            onClick={goBack}
          >
            <ArrowLeft size={14} />
          </button>
          <nav className="view-tabs" aria-label="Inspection views">
            {views.map((item) => (
              <button
                key={item.id}
                ref={item.id === tab ? activeTab : undefined}
                type="button"
                aria-current={item.id === tab ? "page" : undefined}
                onClick={() => {
                  setLastTab(item.id);
                  if (item.id === tab) return;
                  setView(
                    item.id === "entities"
                      ? useReplayStore.getState().entityView
                      : item.id,
                  );
                }}
              >
                {item.label}
              </button>
            ))}
          </nav>
          <div className="replay-controls">
            <button
              type="button"
              className="replay-open"
              aria-label={
                file
                  ? `Open another replay. Current replay: ${file.name}`
                  : "Open replay"
              }
              title={
                file
                  ? [
                      file.name,
                      `${(file.size / 1048576).toFixed(1)} MiB`,
                      parser
                        ? `${parser.totalTicks().toLocaleString()} ticks`
                        : "",
                      "Open another replay (Ctrl+O / Cmd+O)",
                    ]
                      .filter(Boolean)
                      .join("\n")
                  : "Open replay (Ctrl+O / Cmd+O)"
              }
              onClick={open}
            >
              <FolderOpen size={15} aria-hidden="true" />
              <span className="replay-open-label">
                {file?.name ?? "Open replay…"}
              </span>
            </button>
            {file && (
              <button
                type="button"
                className="header-icon-button"
                aria-label="Close replay"
                title="Close replay"
                onClick={() => {
                  fileVersion.current++;
                  setFile(undefined);
                  setError("");
                }}
              >
                <X size={14} />
              </button>
            )}
          </div>
        </div>
        {headerError && (
          <div className="header-error" role="alert">
            {headerError}
          </div>
        )}
      </header>
      <main className="stage">
        <h1 className="sr-only">{pageTitle}</h1>
        {file && !parser && !initError ? (
          <p className="loading" role="status">
            {loadingStage}…
          </p>
        ) : (
          <DemoScreen key={fileVersion.current} />
        )}
      </main>
      <DemoTimeline />
      {isDragActive && <div className="drop-overlay">Drop replay to open</div>}
    </div>
  );
}
