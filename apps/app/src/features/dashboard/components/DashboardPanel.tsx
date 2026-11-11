import type { LucideIcon } from "lucide-react";

type Props = {
  title: string;
  description?: string;
  icon?: LucideIcon;
  children: React.ReactNode;
  className?: string;
};

export function DashboardPanel({
  title,
  description,
  icon: Icon,
  children,
  className,
}: Props) {
  return (
    <section
      className={`overflow-hidden rounded-xl border border-border bg-card ${className ?? ""}`}
    >
      <header className="flex items-center justify-between gap-4 border-b border-border px-5 py-4">
        <div className="min-w-0">
          <h2 className="font-semibold">{title}</h2>

          {description && (
            <p className="mt-1 text-xs text-muted-foreground">{description}</p>
          )}
        </div>

        {Icon && (
          <Icon
            size={19}
            className="shrink-0 text-muted-foreground"
            aria-hidden="true"
          />
        )}
      </header>

      {children}
    </section>
  );
}
