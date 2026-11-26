import { useInspectorPreferences, type TableId } from "../app/preferences";
import {
  useLayoutEffect,
  useRef,
  type CSSProperties,
  type ReactNode,
  type Ref,
} from "react";
import { FieldColumnHeader } from "./FieldColumnHeader";

function useScrollHeader() {
  const ref = useRef<HTMLDivElement>(null);
  useLayoutEffect(() => {
    const header = ref.current;
    const body = header?.nextElementSibling;
    if (!header || !(body instanceof HTMLElement)) return;

    const align = () => {
      header.style.setProperty(
        "--body-scrollbar-width",
        `${body.offsetWidth - body.clientWidth}px`,
      );
      header.scrollLeft = body.scrollLeft;
    };
    let frame = 0;
    const observer = new ResizeObserver(() => {
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(align);
    });
    observer.observe(body);
    align();
    return () => {
      observer.disconnect();
      cancelAnimationFrame(frame);
    };
  }, []);
  return ref;
}

export function IndexHeading({
  label,
  value,
}: {
  label: ReactNode;
  value: ReactNode;
}) {
  const ref = useScrollHeader();
  return (
    <div className="index-heading" ref={ref}>
      {label}
      {value}
    </div>
  );
}

function SheetViewport({
  header,
  viewportRef,
  children,
  contentClass = "",
  className = "",
  style,
}: {
  header: ReactNode;
  viewportRef?: Ref<HTMLDivElement>;
  children: ReactNode;
  contentClass?: string;
  className?: string;
  style?: CSSProperties;
}) {
  const headerRef = useScrollHeader();
  return (
    <div className={`sheet ${className}`} style={style}>
      <div className="table-header" ref={headerRef}>
        <div className={contentClass}>{header}</div>
      </div>
      <div
        className="sheet-scroll"
        ref={viewportRef}
        onScroll={(event) => {
          if (headerRef.current)
            headerRef.current.scrollLeft = event.currentTarget.scrollLeft;
        }}
      >
        <div className={contentClass}>{children}</div>
      </div>
    </div>
  );
}

interface ColumnSize {
  minimum: number;
  grow?: number;
  maximum?: string;
}

export function SheetTable<Column extends string>({
  tableId,
  columns,
  children,
  viewportRef,
  className = "",
  compactIndex = false,
  sizes,
  sort,
  onSort,
}: {
  tableId: TableId;
  columns: readonly Column[];
  sizes?: readonly ColumnSize[];
  sort?: { column: Column; direction: "ascending" | "descending" };
  onSort?: (column: Column) => void;
  children: ReactNode;
  viewportRef?: Ref<HTMLDivElement>;
  className?: string;
  compactIndex?: boolean;
}) {
  const widths = useInspectorPreferences((state) => state.widths?.[tableId]);
  const setColumnWidth = useInspectorPreferences(
    (state) => state.setColumnWidth,
  );
  const defaults = columns.map<ColumnSize>((_, index) => {
    if (sizes?.[index]) return sizes[index];
    if (compactIndex && index === 0) return { minimum: 60 };
    return {
      minimum: index === 0 ? 140 : index === 1 ? 90 : 100,
      grow: index === 0 || (columns.length === 3 && index === 2) ? 2 : 1,
    };
  });
  const tracks = columns.map((column, index) => {
    const width = widths?.[column];
    if (width !== undefined) return width + "px";
    const { minimum, grow, maximum } = defaults[index];
    const limit = maximum ?? (grow ? `${grow}fr` : undefined);
    return limit ? `minmax(${minimum}px, ${limit})` : minimum + "px";
  });
  return (
    <SheetViewport
      className={className}
      contentClass="resizable-sheet"
      viewportRef={viewportRef}
      style={
        {
          "--columns": tracks.join(" "),
          "--sheet-min-width":
            columns.reduce(
              (sum, column, index) =>
                sum + (widths?.[column] ?? defaults[index].minimum),
              0,
            ) + "px",
        } as CSSProperties
      }
      header={
        <div className="sheet-heading sheet-grid">
          {columns.map((column) => (
            <FieldColumnHeader
              key={column}
              label={column}
              direction={sort?.column === column ? sort.direction : undefined}
              onSort={onSort ? () => onSort(column) : undefined}
              onResize={(width) => setColumnWidth(tableId, column, width)}
            />
          ))}
        </div>
      }
    >
      {children}
    </SheetViewport>
  );
}
