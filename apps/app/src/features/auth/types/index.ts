import type z from "zod";

import type { loginSchema, resetPasswordSchema } from "../schemas";

type AuthToken = {
  access_token: string;
  expires_at: Date;
};

type LoginRequestDto = z.infer<typeof loginSchema>;
type ResetPasswordRequestDto = z.infer<typeof resetPasswordSchema>;

export type { AuthToken, LoginRequestDto, ResetPasswordRequestDto };
