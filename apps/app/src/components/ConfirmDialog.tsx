import { X } from "lucide-react";
import { useState } from "react";

import { Button } from "./Button";
import { Loader } from "./Loader";

type Props = {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  description?: string;
  confirmText?: string;
  cancelText?: string;
  variant?: "default" | "destructive";
  onConfirm: () => void | Promise<void>;
};

export function ConfirmDialog({
  isOpen,
  onClose,
  title = "Deseja realizar esta ação?",
  description = "Esta ação pode ser irreversível.",
  confirmText = "Confirmar",
  cancelText = "Cancelar",
  variant = "default",
  onConfirm,
}: Props) {
  const [isLoading, setIsLoading] = useState(false);

  if (!isOpen) return null;

  async function handleConfirm() {
    try {
      setIsLoading(true);
      await onConfirm();
      onClose();
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4 backdrop-blur-sm"
      onMouseDown={(event) => {
        if (event.target === event.currentTarget && !isLoading) {
          onClose();
        }
      }}
    >
      <section
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="confirm-dialog-title"
        aria-describedby="confirm-dialog-description"
        className="relative w-full max-w-md space-y-5 rounded-xl border border-border bg-card p-6 shadow-2xl"
      >
        <button
          type="button"
          onClick={onClose}
          disabled={isLoading}
          aria-label="Fechar"
          className="absolute top-4 right-4 rounded-lg p-2 text-muted-foreground transition-colors hover:bg-accent hover:text-foreground disabled:opacity-50"
        >
          <X size={18} strokeWidth={1.8} />
        </button>

        <div className="space-y-2 pr-8">
          <h2
            id="confirm-dialog-title"
            className="text-lg font-semibold text-foreground"
          >
            {title}
          </h2>

          <p
            id="confirm-dialog-description"
            className="text-sm leading-relaxed text-muted-foreground"
          >
            {description}
          </p>
        </div>

        <div className="flex justify-end gap-3">
          <Button variant="outline" onClick={onClose} disabled={isLoading}>
            {cancelText}
          </Button>

          <Button
            variant={variant}
            onClick={handleConfirm}
            disabled={isLoading}
          >
            {isLoading ? <Loader /> : confirmText}
          </Button>
        </div>
      </section>
    </div>
  );
}
