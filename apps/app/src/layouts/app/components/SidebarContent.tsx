import { LogOut, X } from "lucide-react";
import { Link, NavLink } from "react-router-dom";

import type { Client } from "@/features/clients";

import { Button } from "@/components";
import { navigation, type UserRole } from "../constants";
import { getInitials } from "../utils";

type SidebarContentProps = {
  userName: string;
  email: string;
  role: UserRole;
  client: Client | null;
  openConfirmLogoutModal: () => void;
  onNavigate: () => void;
  onClose?: () => void;
};

export function SidebarContent({
  userName,
  email,
  role,
  client,
  openConfirmLogoutModal,
  onNavigate,
  onClose,
}: SidebarContentProps) {
  const navigationItems = navigation[role];

  return (
    <>
      <div className="flex h-16 items-center justify-between border-b border-sidebar-foreground/10 px-6">
        <Link
          to="/dashboard"
          onClick={onNavigate}
          aria-label="Ir para o painel do Agreega"
        >
          <img src="/images/logo.svg" alt="Agreega" className="h-8 w-auto" />
        </Link>

        {onClose && (
          <button
            type="button"
            onClick={onClose}
            aria-label="Fechar menu"
            className="rounded-md p-2 text-sidebar-foreground/60 transition hover:bg-sidebar-foreground/5 hover:text-sidebar-foreground lg:hidden"
          >
            <X size={16} />
          </button>
        )}
      </div>

      <div className="flex-1 overflow-y-auto px-4 py-6">
        {client && client.client_type === "company" && (
          <div className="mb-8 flex items-center gap-3 rounded-sm border border-sidebar-foreground/10 p-3">
            <div className="flex size-9 shrink-0 items-center justify-center rounded-full bg-sidebar-accent text-xs font-semibold text-sidebar-accent-foreground">
              {getInitials(client.company_name!)}
            </div>

            <p className="truncate text-sm font-medium text-sidebar-foreground">
              {client.company_name!}
            </p>
          </div>
        )}

        <nav className="space-y-2">
          {navigationItems.map(({ label, href, icon: Icon, end }) => (
            <NavLink
              key={href}
              to={href}
              end={end}
              onClick={() => {
                onNavigate();
                if (onClose) onClose();
              }}
              className={({ isActive }) =>
                `flex items-center gap-3 border-l-2 px-3 py-3 text-sm transition-colors ${
                  isActive
                    ? "border-sidebar-accent bg-sidebar-accent/5 font-medium text-sidebar-accent"
                    : "border-transparent text-sidebar-foreground/65 hover:bg-sidebar-foreground/5 hover:text-sidebar-foreground"
                }`
              }
            >
              <Icon size={18} strokeWidth={1.8} />
              <span>{label}</span>
            </NavLink>
          ))}
        </nav>
      </div>

      <div className="border-t border-sidebar-foreground/10 p-4">
        <div className="mb-4 flex items-center gap-3 px-2">
          <div className="flex size-9 shrink-0 items-center justify-center rounded-full bg-sidebar-accent text-xs font-semibold text-sidebar-accent-foreground">
            {getInitials(userName) || "A"}
          </div>

          <div className="min-w-0 flex-1">
            <p className="truncate text-sm font-medium text-sidebar-foreground">
              {userName}
            </p>

            <p className="truncate text-xs text-sidebar-foreground/45">
              {email}
            </p>
          </div>
        </div>

        <Button
          type="button"
          variant="ghost"
          onClick={openConfirmLogoutModal}
          className="w-full justify-start gap-3 rounded-lg px-3 py-2.5 text-sm text-sidebar-foreground/60 hover:bg-sidebar-foreground/5 hover:text-sidebar-foreground"
        >
          <LogOut size={18} />
          <span>Sair da conta</span>
        </Button>
      </div>
    </>
  );
}
