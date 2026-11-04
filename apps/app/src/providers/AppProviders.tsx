import { QueryProvider } from "./QueryProvider";
import { ThemeProvider } from "./ThemeProvider";
import { ToastProvider } from "./ToastProvider";

type Props = {
  children: React.ReactNode;
};

export function AppProviders({ children }: Props) {
  return (
    <QueryProvider>
      <ThemeProvider>
        <ToastProvider>{children}</ToastProvider>
      </ThemeProvider>
    </QueryProvider>
  );
}
