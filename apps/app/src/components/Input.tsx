import type { InputHTMLAttributes } from "react";

type Props = InputHTMLAttributes<HTMLInputElement> & {
  label: string;
  error?: string;
};

export function Input({
  label,
  error,
  required,
  id,
  className,
  ...props
}: Props) {
  return (
    <div className="w-full">
      <label
        htmlFor={id}
        className="mb-2 block text-sm font-medium text-foreground"
      >
        {label}

        {required && <span className="ml-1 text-destructive">*</span>}
      </label>

      <input
        {...props}
        id={id}
        required={required}
        aria-invalid={!!error}
        className={`
          w-full rounded-lg border bg-card px-3 py-2.5
          text-sm text-foreground outline-none transition
          placeholder:text-muted-foreground
          disabled:cursor-not-allowed disabled:opacity-50
          ${
            error
              ? "border-destructive focus:ring-2 focus:ring-destructive/15"
              : "border-input focus:border-ring focus:ring-2 focus:ring-ring/15"
          }
          ${className ?? ""}
        `}
      />

      {error && <p className="mt-1.5 text-xs text-destructive">{error}</p>}
    </div>
  );
}
