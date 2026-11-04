import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";

import { ToastContext, type ToastItem, type ToastVariant } from "@/contexts";

import { Toast } from "@/components";

type Props = {
  children: ReactNode;
};

const DEFAULT_DURATION = 8000;
const MAX_TOASTS = 4;

export function ToastProvider({ children }: Props) {
  const [toasts, setToasts] = useState<ToastItem[]>([]);

  const timers = useRef(new Map<string, ReturnType<typeof setTimeout>>());

  const dismiss = useCallback((id: string) => {
    const timer = timers.current.get(id);

    if (timer) {
      clearTimeout(timer);
      timers.current.delete(id);
    }

    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  }, []);

  const show = useCallback(
    (variant: ToastVariant, message: string) => {
      const id = crypto.randomUUID();

      const newToast: ToastItem = {
        id,
        message,
        variant,
      };

      setToasts((prev) => {
        const next = [...prev, newToast];

        const removed = next.slice(0, Math.max(0, next.length - MAX_TOASTS));

        removed.forEach((toast) => {
          const timer = timers.current.get(toast.id);

          if (timer) {
            clearTimeout(timer);
            timers.current.delete(toast.id);
          }
        });

        return next.slice(-MAX_TOASTS);
      });

      const timer = setTimeout(() => {
        dismiss(id);
      }, DEFAULT_DURATION);

      timers.current.set(id, timer);
    },
    [dismiss],
  );

  const notify = {
    success: (message: string) => show("success", message),
    error: (message: string) => show("error", message),
    warning: (message: string) => show("warning", message),
    info: (message: string) => show("info", message),
  };

  useEffect(() => {
    const currentTimers = timers.current;

    return () => {
      currentTimers.forEach(clearTimeout);
      currentTimers.clear();
    };
  }, []);

  return (
    <ToastContext.Provider value={{ toasts, notify, dismiss }}>
      {children}

      <div className="pointer-events-none fixed right-4 bottom-4 z-100 flex w-[calc(100%-2rem)] max-w-90 flex-col gap-3">
        {toasts.map((toast) => (
          <div key={toast.id} className="pointer-events-auto">
            <Toast toast={toast} onDismiss={dismiss} />
          </div>
        ))}
      </div>
    </ToastContext.Provider>
  );
}
