import type { ButtonHTMLAttributes } from "react";

type Props = ButtonHTMLAttributes<HTMLButtonElement>;

export function Button({ className, children, ...props }: Props) {
  return (
    <button
      {...props}
      className={`
        inline-flex items-center justify-center rounded-lg
        bg-primary px-4 py-2.5 text-sm font-medium
        text-primary-foreground transition
        hover:opacity-90
        focus-visible:outline-none focus-visible:ring-2
        focus-visible:ring-ring focus-visible:ring-offset-2
        disabled:cursor-not-allowed disabled:opacity-50
        ${className ?? ""}
      `}
    >
      {children}
    </button>
  );
}
