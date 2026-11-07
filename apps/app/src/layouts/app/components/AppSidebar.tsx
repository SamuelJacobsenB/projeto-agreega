import type { Client } from "@/features/clients";
import type { UserRole } from "@/features/users";

import { SidebarContent } from "./SidebarContent";

type AppSidebarProps = {
  userName: string;
  email: string;
  role: UserRole;
  client: Client | null;
  isMobileMenuOpen: boolean;
  openConfirmLogoutModal: () => void;
  onCloseMobileMenu: () => void;
};

export function AppSidebar({
  isMobileMenuOpen,
  onCloseMobileMenu,
  ...props
}: AppSidebarProps) {
  return (
    <>
      <aside className="fixed inset-y-0 left-0 z-30 hidden w-64 flex-col bg-sidebar text-sidebar-foreground lg:flex">
        <SidebarContent {...props} onNavigate={onCloseMobileMenu} />
      </aside>

      {isMobileMenuOpen && (
        <div className="fixed inset-0 z-40 lg:hidden">
          <button
            type="button"
            aria-label="Fechar menu"
            onClick={onCloseMobileMenu}
            className="absolute inset-0 bg-black/50"
          />

          <aside className="relative flex h-full w-[min(84vw,280px)] flex-col bg-sidebar text-sidebar-foreground shadow-xl">
            <SidebarContent
              {...props}
              onNavigate={onCloseMobileMenu}
              onClose={onCloseMobileMenu}
            />
          </aside>
        </div>
      )}
    </>
  );
}
