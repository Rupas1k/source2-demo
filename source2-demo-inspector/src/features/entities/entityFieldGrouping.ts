import type { FieldSnapshot } from "../../replay/readFields";
import { compareFieldPaths } from "./entityFieldFormatting";
import type { EntityFieldGroup, EntityFieldRow } from "./entityTypes";

const FIELD_INDEX_RE = /^\d{4}$/;
const ARRAY_TYPE_RE = /^(.*)\[(\d+)\]$/;

function getArrayTypeParts(encodedAs: string) {
  const match = encodedAs.match(ARRAY_TYPE_RE);
  if (!match) {
    return undefined;
  }

  return {
    elementType: match[1],
    length: Number(match[2]),
  };
}

function groupRow(group: EntityFieldGroup, depth: number): EntityFieldRow {
  const decodedAs =
    group.kind === "array"
      ? "Array"
      : group.kind === "vector"
        ? "Vector"
        : "Item";
  return {
    inner: {
      path: group.path,
      namedPath: group.namedPath,
      value: group.kind === "vectorItem" ? "" : String(group.length),
      encodedAs: group.encodedAs,
      decodedAs,
    },
    joinedNamedPath: group.key,
    depth,
    expandableKind: group.kind,
    collectionLength: group.kind === "vectorItem" ? undefined : group.length,
  };
}

export function buildEntityFieldRows(
  rawEntityFieldList: FieldSnapshot[] | undefined,
  expandedFieldGroups: Set<string> | "all",
) {
  const rows: EntityFieldRow[] = [];
  const groups = new Map<string, EntityFieldGroup>();
  const vectorMaxIndexByKey = new Map<string, number>();
  const emittedGroups = new Set<string>();

  const sortedFields = rawEntityFieldList?.slice().sort(compareFieldPaths);
  const isExpanded = (groupKey: string) =>
    expandedFieldGroups === "all" || expandedFieldGroups.has(groupKey);

  for (const entityField of sortedFields ?? []) {
    const arrayType = getArrayTypeParts(entityField.encodedAs);
    for (let i = 1; i < entityField.namedPath.length; i++) {
      const part = entityField.namedPath[i];
      if (!FIELD_INDEX_RE.test(part)) {
        continue;
      }

      const groupNamedPath = entityField.namedPath.slice(0, i);
      const groupKey = groupNamedPath.join(".");

      if (i < entityField.namedPath.length - 1) {
        groups.set(groupKey, {
          key: groupKey,
          path: entityField.path.slice(0, i),
          namedPath: groupNamedPath,
          kind: "vector",
          length: 0,
          encodedAs: "vector",
        });

        vectorMaxIndexByKey.set(
          groupKey,
          Math.max(vectorMaxIndexByKey.get(groupKey) ?? -1, Number(part)),
        );

        const itemNamedPath = entityField.namedPath.slice(0, i + 1);
        const itemKey = itemNamedPath.join(".");
        groups.set(itemKey, {
          key: itemKey,
          path: entityField.path.slice(0, i + 1),
          namedPath: itemNamedPath,
          kind: "vectorItem",
          length: 0,
          encodedAs: "item",
        });
      } else if (arrayType) {
        groups.set(groupKey, {
          key: groupKey,
          path: entityField.path.slice(0, i),
          namedPath: groupNamedPath,
          kind: "array",
          length: arrayType.length,
          encodedAs: entityField.encodedAs,
        });
      } else {
        groups.set(groupKey, {
          key: groupKey,
          path: entityField.path.slice(0, i),
          namedPath: groupNamedPath,
          kind: "vector",
          length: 0,
          encodedAs: "vector",
        });

        vectorMaxIndexByKey.set(
          groupKey,
          Math.max(vectorMaxIndexByKey.get(groupKey) ?? -1, Number(part)),
        );
      }
    }
  }

  for (const [groupKey, maxIndex] of vectorMaxIndexByKey) {
    const group = groups.get(groupKey);
    if (group?.kind === "vector") {
      group.length = maxIndex + 1;
    }
  }

  const emitRow = (row: EntityFieldRow) => {
    rows.push(row);
  };

  for (const entityField of sortedFields ?? []) {
    const arrayType = getArrayTypeParts(entityField.encodedAs);
    let leafEncodedAs = entityField.encodedAs;
    let hiddenByCollapsedGroup = false;
    let depth = 0;

    for (let i = 1; i < entityField.namedPath.length; i++) {
      const part = entityField.namedPath[i];
      if (!FIELD_INDEX_RE.test(part)) {
        continue;
      }

      const groupKey = entityField.namedPath.slice(0, i).join(".");
      const group = groups.get(groupKey);
      if (!group) {
        continue;
      }

      if (!emittedGroups.has(groupKey)) {
        emitRow(groupRow(group, depth));
        emittedGroups.add(groupKey);
      }

      depth += 1;
      if (!isExpanded(groupKey)) {
        hiddenByCollapsedGroup = true;
        break;
      }

      if (
        i === entityField.namedPath.length - 1 &&
        group.kind === "array" &&
        arrayType
      ) {
        leafEncodedAs = arrayType.elementType;
      }

      const itemKey = entityField.namedPath.slice(0, i + 1).join(".");
      const itemGroup = groups.get(itemKey);
      if (itemGroup?.kind !== "vectorItem") {
        continue;
      }

      if (!emittedGroups.has(itemKey)) {
        emitRow(groupRow(itemGroup, depth));
        emittedGroups.add(itemKey);
      }

      depth += 1;
      if (!isExpanded(itemKey)) {
        hiddenByCollapsedGroup = true;
        break;
      }
    }

    if (hiddenByCollapsedGroup) {
      continue;
    }

    emitRow({
      inner: entityField,
      joinedNamedPath: entityField.namedPath.join("."),
      depth,
      expandableKind: undefined,
    });

    if (leafEncodedAs !== entityField.encodedAs) {
      rows[rows.length - 1] = {
        ...rows[rows.length - 1],
        inner: {
          ...entityField,
          encodedAs: leafEncodedAs,
        },
      };
    }
  }

  return rows;
}

export function filterEntityFieldRows(
  rows: EntityFieldRow[],
  searchCmpFn: (value: string) => boolean,
) {
  const rowsByPath = new Map(rows.map((row) => [row.joinedNamedPath, row]));
  const includedPaths = new Set<string>();

  for (const row of rows) {
    if (!searchCmpFn(row.joinedNamedPath)) {
      continue;
    }

    for (let i = 1; i <= row.inner.namedPath.length; i++) {
      const parentPath = row.inner.namedPath.slice(0, i).join(".");
      if (rowsByPath.has(parentPath)) {
        includedPaths.add(parentPath);
      }
    }
  }

  return rows.filter((row) => includedPaths.has(row.joinedNamedPath));
}
