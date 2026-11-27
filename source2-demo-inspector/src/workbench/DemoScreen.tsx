import {
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { views } from "../app/views";
import EntitiesPanel from "../features/entities/EntitiesPanel";
import GameEventsPanel from "../features/gameEvents/GameEventsPanel";
import MessagesPanel from "../features/messages/MessagesPanel";
import StringTablesPanel from "../features/stringTables/StringTablesPanel";
import { useReplayStore } from "../replay/state";

type Tab = (typeof views)[number]["id"];
const panels = {
  entities: EntitiesPanel,
  stringTables: StringTablesPanel,
  gameEvents: GameEventsPanel,
  messages: MessagesPanel,
};

export default function DemoScreen() {
  const view = useReplayStore((state) => state.activeView);
  const tab = view === "baselineEntities" ? "entities" : view;
  const [visited, setVisited] = useState<ReadonlySet<Tab>>(
    () => new Set([tab]),
  );
  useEffect(() => {
    setVisited((previous) =>
      previous.has(tab) ? previous : new Set([...previous, tab]),
    );
  }, [tab]);

  return views.map(({ id }) => {
    if (id !== tab && !visited.has(id)) return null;
    const Panel = panels[id];
    return (
      <PreservedPane key={id} visible={id === tab}>
        <Panel />
      </PreservedPane>
    );
  });
}

function PreservedPane({
  visible,
  children,
}: {
  visible: boolean;
  children: ReactNode;
}) {
  const scrollPositions = useRef(
    new Map<HTMLElement, { top: number; left: number }>(),
  );
  useLayoutEffect(() => {
    if (!visible) return;
    const frame = requestAnimationFrame(() => {
      for (const [element, position] of scrollPositions.current) {
        if (!element.isConnected) scrollPositions.current.delete(element);
        else if (element.getClientRects().length) element.scrollTo(position);
      }
    });
    return () => cancelAnimationFrame(frame);
  }, [visible]);

  return (
    <div
      className="pane-layout"
      hidden={!visible}
      onScrollCapture={(event) => {
        const element = event.target;
        if (
          visible &&
          element instanceof HTMLElement &&
          element.matches(".sheet-scroll") &&
          element.getClientRects().length
        ) {
          scrollPositions.current.set(element, {
            top: element.scrollTop,
            left: element.scrollLeft,
          });
        }
      }}
    >
      {children}
    </div>
  );
}
