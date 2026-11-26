import { readMessageData as read } from "./readMessageData";
import { useEffect, useState } from "react";
import type {
  MessageInspector,
  ReplaySession,
} from "../../generated/wasm/inspector";
import { useReplayStore } from "../../replay/state";
import { RecordPanes } from "../../workbench/RecordPanes";

import { MessageList } from "./MessageList";
import { MessageDetails } from "./MessageDetails";
import { useMessageNavigation } from "./useMessageNavigation";

function TickMessages(props: Parameters<typeof useMessageNavigation>[0]) {
  const { listProps, detailsProps } = useMessageNavigation(props);
  return (
    <div className="pane-layout">
      <RecordPanes
        recordTitle="Messages"
        detailTitle="Protobuf fields"
        records={<MessageList {...listProps} />}
        details={<MessageDetails {...detailsProps} />}
      />
    </div>
  );
}

export default function MessagesPanel() {
  const session = useReplayStore((state) => state.replaySession);
  const parserTick = useReplayStore((state) => state.currentTick);
  const messageTick = useReplayStore((state) => state.selectedMessageTick);
  const tick = messageTick ?? parserTick;
  const selectTick = useReplayStore((state) => state.setSelectedMessageTick);
  const [opened, setOpened] = useState<{
    session: ReplaySession;
    value?: MessageInspector;
    error: string;
  }>();
  useEffect(() => {
    if (!session) return;
    const result = read(() => session.openMessages(), undefined);
    setOpened({ session, ...result });
    if (useReplayStore.getState().selectedMessageTick === undefined) {
      selectTick(useReplayStore.getState().currentTick);
    }
    return () => {
      result.value?.free();
      selectTick(undefined);
    };
  }, [session, selectTick]);
  const active = opened?.session === session ? opened : undefined;
  return (
    <TickMessages
      key={String(!!active)}
      tick={tick}
      inspector={active?.value}
      error={active?.error ?? ""}
    />
  );
}
