import { z } from "zod";

import { isValidCpf } from "@/lib/validation";

export const userRoleSchema = z.enum([
  "admin",
  "engineer",
  "architect",
  "technical",
  "client",
]);

export const updateUserSchema = z.object({
  name: z.string().trim().min(1).nullable().optional(),
  phone: z.string().nullable().optional(),
  cpf: z.string().refine(isValidCpf, "CPF inválido.").nullable().optional(),
});
