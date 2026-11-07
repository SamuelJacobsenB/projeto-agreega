import type { ButtonHTMLAttributes } from "react";

type ButtonVariant =
  "default" | "secondary" | "outline" | "ghost" | "destructive";

type ButtonSize = "sm" | "md" | "lg";

type Props = ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: ButtonVariant;
  size?: ButtonSize;
};

const variants: Record<ButtonVariant, string> = {
  default: `
    bg-primary text-primary-foreground
    hover:opacity-90
  `,
  secondary: `
    bg-secondary text-secondary-foreground
    hover:bg-secondary/80
  `,
  outline: `
    border border-border bg-transparent text-foreground
    hover:bg-accent hover:text-accent-foreground
  `,
  ghost: `
    bg-transparent text-foreground
    hover:bg-accent hover:text-accent-foreground
  `,
  destructive: `
    bg-destructive text-destructive-foreground
    hover:bg-destructive/90
  `,
};

const sizes: Record<ButtonSize, string> = {
  sm: "px-3 py-1.5 text-xs",
  md: "px-4 py-2.5 text-sm",
  lg: "px-6 py-3 text-base",
};

export function Button({
  className,
  children,
  variant = "default",
  size = "md",
  ...props
}: Props) {
  return (
    <button
      {...props}
      className={`inline-flex items-center justify-center gap-2 rounded-lg font-medium transition-colors focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none disabled:cursor-not-allowed disabled:opacity-50 ${variants[variant]} ${sizes[size]} ${className ?? ""} `}
    >
      {children}
    </button>
  );
}
