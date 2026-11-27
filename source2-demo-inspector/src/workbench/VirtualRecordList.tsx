import { VirtualRows } from "./VirtualRows";

type Props = Parameters<typeof VirtualRows>[0];

export function VirtualRecordList(props: Props) {
  return (
    <div className="sheet-scroll virtual-record-viewport" ref={props.viewport}>
      <VirtualRows {...props} />
    </div>
  );
}
