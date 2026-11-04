import { createBrowserRouter } from "react-router-dom";

import {
  ConfirmPasswordResetPage,
  LoginPage,
  RequestPasswordResetPage,
} from "@/features/auth";
import { AuthLayout } from "@/layouts";

import { paths } from "./paths";

export const router = createBrowserRouter([
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
]);
