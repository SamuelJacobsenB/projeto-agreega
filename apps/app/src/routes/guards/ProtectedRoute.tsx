import { Navigate, Outlet, useLocation } from "react-router-dom";

import { useAuth } from "@/hooks";
import { LoadingPage } from "@/layouts";
import { paths } from "@/routes/paths";

export function ProtectedRoute() {
  const { isAuthenticated, isLoading } = useAuth();
  const location = useLocation();

  if (isLoading) {
    return <LoadingPage />;
  }

  if (!isAuthenticated) {
    return (
      <Navigate to={paths.auth.login} replace state={{ from: location }} />
    );
  }

  return <Outlet />;
}
