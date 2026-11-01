import { z } from "zod";

export const passwordSchema = z
  .string()
  .min(8, "A senha deve ter pelo menos 8 caracteres.")
  .max(64, "A senha deve ter no máximo 64 caracteres.")
  .refine((value) => value === value.trim(), {
    message: "A senha não pode começar ou terminar com espaços.",
  })
  .regex(/[A-Z]/, "A senha deve conter pelo menos uma letra maiúscula.")
  .regex(/[a-z]/, "A senha deve conter pelo menos uma letra minúscula.")
  .regex(/[0-9]/, "A senha deve conter pelo menos um número.")
  .regex(
    /[^a-zA-Z0-9]/,
    "A senha deve conter pelo menos um caractere especial.",
  );

export const loginSchema = z.object({
  email: z.email("Informe um e-mail válido."),
  password: z.string().min(1, "Informe sua senha."),
});

export const resetPasswordSchema = z.object({
  token: z
    .string()
    .length(64, "Token inválido.")
    .regex(/^[a-f0-9]{64}$/, "Token inválido."),
  password: passwordSchema,
});

export type LoginRequestDto = z.infer<typeof loginSchema>;
export type ResetPasswordRequestDto = z.infer<typeof resetPasswordSchema>;
