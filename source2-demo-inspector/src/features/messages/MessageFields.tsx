import { sortRows, type TableSort } from "../../workbench/tableSort";
import { useState } from "react";

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

function FieldRow({ field, depth }: { field: Field; depth: number }) {
  const [open, setOpen] = useState(field.openByDefault ?? false);
  const expandable = !!field.children?.length;
  return (
    <>
      <div className="sheet-grid message-field">
        <span
          className="truncate"
          title={field.name}
          style={{ paddingLeft: 9 + depth * 12 }}
        >
          {expandable ? (
            <button
              className="block max-w-full truncate"
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
              {open ? "▾" : "▸"} {field.name}
            </button>
          ) : (
            field.name
          )}
        </span>
        <span className="field-type" title={field.kind || undefined}>
          {field.kind}
        </span>
        <span className="field-value" title={field.value || undefined}>
          {field.value}
        </span>
      </div>
      {open && field.children && (
        <FieldRows fields={field.children} depth={depth + 1} />
      )}
    </>
  );
}

export function FieldRows({
  fields,
  depth = 0,
}: {
  fields: Field[];
  depth?: number;
}) {
  const [limit, setLimit] = useState(128);
  return (
    <>
      {fields.slice(0, limit).map((field, i) => (
        <FieldRow key={i} field={field} depth={depth} />
      ))}
      {fields.length > limit && (
        <button className="record-row" onClick={() => setLimit(limit + 128)}>
          Show more fields
        </button>
      )}
    </>
  );
}
