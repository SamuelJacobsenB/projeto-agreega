import { z } from "zod";

import { isValidCpf } from "@/lib/validation";
import { userRoleSchema } from "@/features/users/schemas";

const invitationPasswordSchema = z
  .string()
  .min(8)
  .max(32)
  .refine((value) => value === value.trim())
  .regex(/[A-Z]/)
  .regex(/[a-z]/)
  .regex(/[0-9]/)
  .regex(/[^a-zA-Z0-9]/);

export const createInvitationSchema = z.object({
  email: z.email().max(255),
  client_id: z.uuid().nullable().optional(),
  role: userRoleSchema,
});

export const acceptInvitationSchema = z.object({
  token: z.string().trim().min(1),
  name: z.string().trim().min(2).max(100),
  phone: z.string().regex(/^\d{10,11}$/, "Telefone inválido."),
  cpf: z.string().refine(isValidCpf, "CPF inválido.").nullable().optional(),
  password: invitationPasswordSchema,
});
