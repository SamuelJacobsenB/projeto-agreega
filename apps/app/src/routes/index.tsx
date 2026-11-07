import { createBrowserRouter, Navigate } from "react-router-dom";

import {
  ConfirmPasswordResetPage,
  LoginPage,
  RequestPasswordResetPage,
} from "@/features/auth";
import { DashboardPage } from "@/features/dashboard";
import { AppLayout, AuthLayout } from "@/layouts";

import { ProtectedRoute } from "./guards/ProtectedRoute";
import { paths } from "./paths";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <Navigate to={paths.app.dashboard} replace />,
  },
  {
    element: <AuthLayout />,
    children: [
      {
        path: paths.auth.login,
        element: <LoginPage />,
      },
      {
        path: paths.auth.requestPasswordReset,
        element: <RequestPasswordResetPage />,
      },
      {
        path: paths.auth.confirmPasswordReset,
        element: <ConfirmPasswordResetPage />,
      },
    ],
  },
  {
    element: <ProtectedRoute />,
    children: [
      {
        element: <AppLayout />,
        children: [
          {
            path: paths.app.dashboard,
            element: <DashboardPage />,
          },
        ],
      },
    ],
  },
]);
