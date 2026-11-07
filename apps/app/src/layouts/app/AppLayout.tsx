import { useState } from "react";
import { Navigate, Outlet, useNavigate } from "react-router-dom";

import { useLogout } from "@/features/auth/hooks";
import { useAuth, useTheme, useToast } from "@/hooks";
import { getApiError } from "@/lib";
import { paths } from "@/routes/paths";

import { ConfirmDialog } from "@/components";
import { LoadingPage } from "../LoadingPage";
import { AppHeader, AppSidebar } from "./components";

export function AppLayout() {
  const { session, isLoading, refreshSession } = useAuth();
  const { resolvedTheme, setTheme } = useTheme();
  const { notify } = useToast();

  const { logout } = useLogout();

  const navigate = useNavigate();

  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isConfirmLogoutModalOpen, setIsConfirmLogoutModalOpen] =
    useState(false);

  if (isLoading) {
    return <LoadingPage />;
  }

  if (!session?.user) {
    return <Navigate to={paths.auth.login} replace />;
  }

  const { user, client } = session;

  async function handleLogout() {
    try {
      await logout();
      await refreshSession();

      notify.success("Você saiu da sua conta.");
      navigate(paths.auth.login, { replace: true });
    } catch (error) {
      notify.error(getApiError(error).message);
    }
  }

  function handleToggleTheme() {
    setTheme(resolvedTheme === "dark" ? "light" : "dark");
  }

  return (
    <div className="min-h-dvh bg-background text-foreground">
      <AppSidebar
        userName={user.name || "Usuário"}
        email={user.email}
        role={user.role}
        client={client}
        isMobileMenuOpen={isMobileMenuOpen}
        openConfirmLogoutModal={() => setIsConfirmLogoutModalOpen(true)}
        onCloseMobileMenu={() => setIsMobileMenuOpen(false)}
      />

      <div className="min-h-dvh lg:pl-64">
        <AppHeader
          userName={user.name}
          role={user.role}
          theme={resolvedTheme}
          onToggleTheme={handleToggleTheme}
          onOpenMobileMenu={() => setIsMobileMenuOpen(true)}
        />

        <main className="mx-auto w-full max-w-360 px-5 py-8 sm:px-8 sm:py-10 lg:px-10">
          <Outlet />
        </main>
      </div>

      <ConfirmDialog
        isOpen={isConfirmLogoutModalOpen}
        onClose={() => setIsConfirmLogoutModalOpen(false)}
        title="Sair da conta"
        description="Ao sair da conta, você terá que fazer login novamente para acessar o sistema."
        variant="destructive"
        onConfirm={handleLogout}
      />
    </div>
  );
}
