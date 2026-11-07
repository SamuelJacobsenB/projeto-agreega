import { ChevronRight, Menu, Moon, Sun } from "lucide-react";
import { useLocation } from "react-router-dom";

import type { UserRole } from "@/features/users";

import { navigation } from "../constants";
import { getInitials } from "../utils";

type AppHeaderProps = {
  userName: string;
  role: UserRole;
  theme: "light" | "dark";
  onToggleTheme: () => void;
  onOpenMobileMenu: () => void;
};

export function AppHeader({
  userName,
  role,
  theme,
  onToggleTheme,
  onOpenMobileMenu,
}: AppHeaderProps) {
  const { pathname } = useLocation();

  const activeItem = navigation[role].find((item) =>
    item.end
      ? pathname === item.href
      : pathname === item.href || pathname.startsWith(`${item.href}/`),
  );

  const activeLabel = activeItem?.label ?? "Agreega";

  return (
    <header className="sticky top-0 z-20 flex h-16 items-center justify-between border-b border-border bg-background/95 px-4 backdrop-blur sm:px-8">
      <div className="flex min-w-0 items-center gap-6">
        <button
          type="button"
          onClick={onOpenMobileMenu}
          aria-label="Abrir menu"
          className="flex size-9 shrink-0 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:bg-accent hover:text-foreground lg:hidden"
        >
          <Menu size={20} strokeWidth={1.8} />
        </button>

        <nav
          aria-label="Navegação estrutural"
          className="flex min-w-0 items-center gap-2 text-sm"
        >
          <span className="hidden shrink-0 text-muted-foreground lg:inline">
            Agreega
          </span>

          <ChevronRight
            size={15}
            className="hidden! shrink-0 text-muted-foreground lg:block!"
          />

          <span className="truncate font-medium text-foreground">
            {activeLabel}
          </span>
        </nav>
      </div>

      <div className="flex shrink-0 items-center gap-3">
        <button
          type="button"
          onClick={onToggleTheme}
          aria-label={
            theme === "dark" ? "Ativar tema claro" : "Ativar tema escuro"
          }
          title={theme === "dark" ? "Tema claro" : "Tema escuro"}
          className="flex size-9 items-center justify-center rounded-lg border border-border text-muted-foreground transition-colors hover:bg-accent hover:text-foreground"
        >
          {theme === "dark" ? (
            <Sun size={17} strokeWidth={1.8} />
          ) : (
            <Moon size={17} strokeWidth={1.8} />
          )}
        </button>

        <div
          title={userName}
          aria-label={userName}
          className="flex size-9 shrink-0 items-center justify-center rounded-full bg-primary text-xs font-semibold text-primary-foreground"
        >
          {getInitials(userName) || "A"}
        </div>
      </div>
    </header>
  );
}
