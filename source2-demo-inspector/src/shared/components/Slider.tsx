import { useRef, type CSSProperties, type KeyboardEventHandler } from "react";

interface Props {
  className?: string;
  title?: string;
  disabled?: boolean;
  min: number;
  max: number;
  value: number;
  onValueChange: (value: number) => void;
  onValueCommit: (value: number) => void;
  onKeyDownCapture?: KeyboardEventHandler<HTMLInputElement>;
}

export function Slider({
  className,
  title,
  disabled,
  min,
  max,
  value,
  onValueChange,
  onValueCommit,
  onKeyDownCapture,
}: Props) {
  const pending = useRef<number | undefined>(undefined);
  const percent = (100 * (value - min)) / Math.max(1, max - min);
  function commit() {
    if (pending.current === undefined) return;
    const next = pending.current;
    pending.current = undefined;
    onValueCommit(next);
  }

  return (
    <div
      className={className}
      title={title}
      data-disabled={disabled || undefined}
    >
      <div className="slider-rail" aria-hidden="true">
        <span className="slider-progress" style={{ width: `${percent}%` }} />
      </div>
      <span
        aria-hidden="true"
        className="slider-thumb"
        style={{ "--slider-position": `${percent}%` } as CSSProperties}
      />
      <input
        type="range"
        className="slider-input"
        role="slider"
        aria-label="Replay tick"
        aria-valuemin={min}
        aria-valuemax={max}
        aria-valuenow={value}
        min={min}
        max={max}
        value={value}
        disabled={disabled}
        onChange={(event) => {
          pending.current = event.currentTarget.valueAsNumber;
          onValueChange(pending.current);
        }}
        onPointerUp={commit}
        onLostPointerCapture={commit}
        onKeyDownCapture={onKeyDownCapture}
        onKeyUp={commit}
        onBlur={commit}
      />
    </div>
  );
}
