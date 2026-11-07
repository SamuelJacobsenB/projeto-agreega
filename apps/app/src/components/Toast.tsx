import { CircleCheck, CircleX, Info, TriangleAlert } from "lucide-react";

import type { ToastItem } from "@/contexts";

type Props = {
  toast: ToastItem;
  onDismiss: (id: string) => void;
};

const variants = {
  success: {
    icon: CircleCheck,
    iconClass: "text-emerald-500",
  },
  error: {
    icon: CircleX,
    iconClass: "text-destructive",
  },
  warning: {
    icon: TriangleAlert,
    iconClass: "text-amber-500",
  },
  info: {
    icon: Info,
    iconClass: "text-blue-500",
  },
};

export function Toast({ toast, onDismiss }: Props) {
  const { icon: Icon, iconClass } = variants[toast.variant];

  return (
    <div
      role={toast.variant === "error" ? "alert" : "status"}
      className="animate-toast flex w-fit items-start gap-3 rounded-lg border border-border bg-card p-4 text-card-foreground shadow-sm"
    >
      <Icon
        size={20}
        strokeWidth={1.8}
        className={`mt-0.5 shrink-0 ${iconClass}`}
      />

      <p className="text-sm font-medium">{toast.message}</p>

      <button
        type="button"
        onClick={() => onDismiss(toast.id)}
        aria-label="Fechar notificação"
        className="text-lg text-muted-foreground hover:text-foreground"
      >
        ✕
      </button>
    </div>
  );
}
