import {
  CheckIcon,
  CopyIcon,
  ChevronDownIcon,
  ChevronRightIcon,
  Link2Icon,
  Link2OffIcon,
  HistoryIcon,
} from "../../shared/components/Icons";
import { eHandleToIndex, isEHandleValid } from "../../generated/wasm/inspector";
import { useState, type CSSProperties } from "react";
import { itemCount } from "../../shared/utils/formatNumber";
import type { EntityFieldRow } from "./entityTypes";

interface Props {
  row: EntityFieldRow;
  expanded: boolean;
  comparing: boolean;
  previousValue?: string;
  removed: boolean;
  partial?: boolean;
  onToggle: (path: string) => void;
  onFollow: (index: number, handle: number) => void;
  onSource?: () => void;
}
export function FieldCells({
  row,
  expanded,
  comparing,
  previousValue,
  removed,
  partial,
  onToggle,
  onFollow,
  onSource,
}: Props) {
  const { inner } = row;
  const segments = inner.namedPath;
  const [copyStatus, setCopyStatus] = useState<{
    path: string;
    copied: boolean;
  }>();
  const path = row.joinedNamedPath;
  const copied = copyStatus?.path === path && copyStatus.copied;
  async function copyPath() {
    try {
      await navigator.clipboard.writeText(path);
      setCopyStatus({ path, copied: true });
    } catch {
      setCopyStatus({ path, copied: false });
    }
  }
  const handle =
    !row.expandableKind &&
    inner.encodedAs.startsWith("CHandle") &&
    inner.value !== "";
  const valid = handle && isEHandleValid(+inner.value);
  return (
    <>
      <div className="field-name">
        <div
          className="field-path"
          style={{ "--field-depth": row.depth } as CSSProperties}
        >
          {row.expandableKind ? (
            <button
              type="button"
              className="field-disclosure"
              aria-label={row.joinedNamedPath}
              aria-expanded={expanded}
              onClick={() => onToggle(row.joinedNamedPath)}
              onKeyDown={(event) => {
                if (
                  event.altKey ||
                  (event.key !== "ArrowRight" && event.key !== "ArrowLeft")
                )
                  return;
                event.preventDefault();
                if ((event.key === "ArrowRight") !== expanded)
                  onToggle(row.joinedNamedPath);
              }}
            >
              {expanded ? (
                <ChevronDownIcon size={13} />
              ) : (
                <ChevronRightIcon size={13} />
              )}
            </button>
          ) : (
            <span className="field-toggle-spacer" aria-hidden="true" />
          )}
          <span className="field-path-text truncate" title={path}>
            {segments.slice(0, -1).map((part, i) => (
              <span className="path-parent" key={i}>
                {part}.
              </span>
            ))}
            <span className="path-leaf">{segments[segments.length - 1]}</span>
          </span>
          <span className="field-path-actions">
            <button
              type="button"
              className="field-copy"
              aria-label={`Copy field path: ${path}`}
              title={copied ? "Copied" : "Copy field path"}
              onClick={() => void copyPath()}
            >
              {copied ? <CheckIcon size={13} /> : <CopyIcon size={13} />}
            </button>
            {onSource && (
              <button
                type="button"
                title="Find field source"
                aria-label={`Find field source for ${path}`}
                onClick={onSource}
              >
                <HistoryIcon size={13} />
              </button>
            )}
          </span>
          <span className="sr-only" role="status">
            {copyStatus?.path === path &&
              (copied ? "Field path copied" : "Could not copy field path")}
          </span>
        </div>
      </div>
      <span className="field-type" title={inner.encodedAs}>
        {inner.encodedAs}
      </span>
      <span
        className="field-type"
        title={inner.decodedAs === "None" ? undefined : inner.decodedAs}
      >
        {inner.decodedAs === "None" ? "" : inner.decodedAs}
      </span>
      <div
        className="field-value"
        title={
          comparing
            ? `${previousValue ?? "absent"} → ${removed ? "absent" : inner.value}`
            : inner.value || undefined
        }
      >
        {comparing && (
          <span className="prior-value">{previousValue ?? "absent"} → </span>
        )}
        {comparing && removed ? (
          "absent"
        ) : valid ? (
          <button
            type="button"
            title="Inspect linked entity"
            onClick={() => onFollow(eHandleToIndex(+inner.value), +inner.value)}
          >
            {inner.value}
            <Link2Icon size={13} />
          </button>
        ) : row.expandableKind ? (
          row.expandableKind === "vectorItem" ||
          (partial && row.expandableKind === "vector") ? (
            ""
          ) : (
            itemCount(Number(row.collectionLength ?? inner.value))
          )
        ) : (
          <>
            {inner.value}
            {handle && <Link2OffIcon className="ml-1" size={13} />}
          </>
        )}
      </div>
    </>
  );
}
