import { createContext } from "react";

export type ToastVariant = "success" | "error" | "warning" | "info";

export type ToastItem = {
  id: string;
  message: string;
  variant: ToastVariant;
};

export type ToastContextValue = {
  toasts: ToastItem[];
  notify: {
    success: (message: string) => void;
    error: (message: string) => void;
    warning: (message: string) => void;
    info: (message: string) => void;
  };
  dismiss: (id: string) => void;
};

export const ToastContext = createContext<ToastContextValue | null>(null);
