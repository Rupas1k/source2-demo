import type { Ref } from "react";

export function MessageBranchRows<Row>({
  branch,
  label,
  loading,
  depth,
  activeRef,
  describe,
}: {
  branch?: { changes: Row[]; error?: string };
  label: string;
  loading: string;
  depth: number;
  activeRef: Ref<HTMLButtonElement>;
  describe: (
    row: Row,
    index: number,
  ) => {
    name: string;
    id: number;
    status?: string;
    active: boolean;
    select: () => void;
  };
}) {
  return (
    <ul aria-label={label}>
      {!branch && (
        <li className="message-entity-status" role="status">
          {loading}
        </li>
      )}
      {branch?.error && (
        <li className="message-entity-status" role="alert">
          {branch.error}
        </li>
      )}
      {branch?.changes.map((row, index) => {
        const item = describe(row, index);
        return (
          <li
            key={index}
            className={
              "record-row record-row__selectable message-list-row" +
              (item.active ? " record-row__selected" : "")
            }
            data-depth={depth}
          >
            <span className="message-disclosures" aria-hidden="true" />
            <button
              ref={item.active ? activeRef : undefined}
              type="button"
              className="message-select"
              aria-pressed={item.active}
              onClick={item.select}
            >
              <span className="truncate" title={item.name}>
                {item.name}
              </span>
              {item.status && (
                <span className="record-status">{item.status}</span>
              )}
              <span className="entity-id">{item.id}</span>
            </button>
          </li>
        );
      })}
    </ul>
  );
}
