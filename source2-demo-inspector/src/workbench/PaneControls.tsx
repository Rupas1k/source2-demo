import type { ReactNode } from "react";

interface SearchProps {
  label: string;
  placeholder?: string;
  value: string;
  onChange: (value: string) => void;
}

export function SearchField({
  label,
  placeholder,
  value,
  onChange,
}: SearchProps) {
  return (
    <input
      className="pane-search-field"
      aria-label={label}
      placeholder={placeholder ?? label + "…"}
      value={value}
      spellCheck={false}
      onChange={(event) => onChange(event.target.value)}
      onKeyDown={(event) => {
        if (event.key === "Escape" && value) {
          event.preventDefault();
          onChange("");
        }
      }}
    />
  );
}

export function SearchBar({
  children,
  matchCount,
  ...search
}: SearchProps & {
  children?: ReactNode;
  matchCount?: number;
}) {
  return (
    <div className="field-tools" role="search" aria-label={search.label}>
      <SearchField {...search} />
      {matchCount !== undefined && (
        <output
          className="match-count"
          aria-label="Matches"
          title={`${matchCount.toLocaleString()} ${matchCount === 1 ? "match" : "matches"}`}
        >
          {matchCount.toLocaleString()}
        </output>
      )}
      {children}
    </div>
  );
}

export function FilterToggle({
  pressed,
  onClick,
  children,
  title,
}: {
  pressed: boolean;
  onClick: () => void;
  children: ReactNode;
  title?: string;
}) {
  return (
    <button
      type="button"
      className="filter-toggle"
      aria-pressed={pressed}
      onClick={onClick}
      title={title}
    >
      {children}
    </button>
  );
}
