import { ChevronDown, ChevronRight } from "../../shared/components/Icons";
import { sortRows, type TableSort } from "../../workbench/tableSort";
import { useState, type CSSProperties } from "react";

export interface Field {
  name: string;
  kind: string;
  value: string;
  children?: Field[];
  openByDefault?: boolean;
}

export function filterMessageFields(fields: Field[], query: string): Field[] {
  const term = query.trim().toLowerCase();
  if (!term) return fields;

  return fields.flatMap((field) => {
    if (
      [field.name, field.kind, field.value].some((value) =>
        value.toLowerCase().includes(term),
      )
    ) {
      return [field];
    }
    const children =
      field.children && filterMessageFields(field.children, term);
    return children?.length
      ? [{ ...field, children, openByDefault: true }]
      : [];
  });
}

export function sortMessageFields(
  fields: Field[],
  sort: TableSort | undefined,
): Field[] {
  if (!sort) return fields;
  return sortRows(fields, sort, (field, column) =>
    column === "Field"
      ? field.name
      : column === "Type"
        ? field.kind
        : field.value,
  ).map((field) =>
    field.children
      ? { ...field, children: sortMessageFields(field.children, sort) }
      : field,
  );
}

function FieldRow({
  field,
  depth,
  path,
}: {
  field: Field;
  depth: number;
  path: string;
}) {
  const [open, setOpen] = useState(field.openByDefault ?? false);
  const expandable = !!field.children?.length;
  const DisclosureIcon = open ? ChevronDown : ChevronRight;
  return (
    <>
      <div className="sheet-grid message-field">
        <span
          className="protobuf-field-name"
          style={{ "--field-depth": depth } as CSSProperties}
        >
          {expandable ? (
            <button
              type="button"
              className="field-disclosure"
              aria-label={field.name}
              aria-expanded={open}
              onKeyDown={(event) => {
                if (
                  event.altKey ||
                  (event.key !== "ArrowRight" && event.key !== "ArrowLeft")
                )
                  return;
                event.preventDefault();
                setOpen(event.key === "ArrowRight");
              }}
              onClick={() => setOpen(!open)}
            >
              <DisclosureIcon size={13} />
            </button>
          ) : (
            <span className="field-disclosure-space" aria-hidden="true" />
          )}
          <span className="truncate" title={path}>
            {field.name}
          </span>
        </span>
        <span className="field-type" title={field.kind || undefined}>
          {field.kind}
        </span>
        <span className="field-value" title={field.value || undefined}>
          {field.value}
        </span>
      </div>
      {open && field.children && (
        <FieldRows
          fields={field.children}
          depth={depth + 1}
          parentPath={path}
        />
      )}
    </>
  );
}

export function FieldRows({
  fields,
  depth = 0,
  parentPath,
}: {
  fields: Field[];
  depth?: number;
  parentPath?: string;
}) {
  const [limit, setLimit] = useState(128);
  return (
    <>
      {fields.slice(0, limit).map((field, i) => (
        <FieldRow
          key={i}
          field={field}
          depth={depth}
          path={parentPath ? `${parentPath}.${field.name}` : field.name}
        />
      ))}
      {fields.length > limit && (
        <button className="record-row" onClick={() => setLimit(limit + 128)}>
          Show more fields
        </button>
      )}
    </>
  );
}
