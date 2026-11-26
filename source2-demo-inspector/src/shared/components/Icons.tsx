import type { SVGProps } from "react";

type IconProps = SVGProps<SVGSVGElement> & { size?: number };

function icon(path: string) {
  return function Icon({ size = 16, ...props }: IconProps) {
    return (
      <svg
        aria-hidden="true"
        width={size}
        height={size}
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="1.75"
        strokeLinecap="round"
        strokeLinejoin="round"
        {...props}
      >
        <path d={path} />
      </svg>
    );
  };
}

export const ArrowLeft = icon("M19 12H5m6-6-6 6 6 6");
export const FolderOpen = icon(
  "M3 19V6a2 2 0 0 1 2-2h5l2 3h7a2 2 0 0 1 2 2M3 19l3-8h16l-3 8Z",
);
export const X = icon("m6 6 12 12M18 6 6 18");
export const ChevronLeft = icon("m15 6-6 6 6 6");
export const ChevronRight = icon("m9 6 6 6-6 6");
export const ChevronDown = icon("m6 9 6 6 6-6");
export const ChevronRightIcon = ChevronRight;
export const ChevronDownIcon = ChevronDown;
export const SkipBack = icon("M5 6v12m14-12L9 12l10 6Z");
export const SkipForward = icon("M19 6v12M5 6l10 6-10 6Z");
export const CheckIcon = icon("m5 12 4 4L19 6");
export const CopyIcon = icon("M8 8h12v12H8ZM4 16V4h12");
export const Link2Icon = icon(
  "M9 15l6-6m-8 3-2 2a4 4 0 0 0 6 6l2-2m-2-12 2-2a4 4 0 0 1 6 6l-2 2",
);
export const Link2OffIcon = icon(
  "m3 3 18 18M7 12l-2 2a4 4 0 0 0 6 6l2-2m-2-12 2-2a4 4 0 0 1 6 6l-2 2",
);
export const HistoryIcon = icon("M3 12a9 9 0 1 1 2 6M3 6v6h6m3-5v5l4 2");
