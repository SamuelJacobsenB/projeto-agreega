import { z } from "zod";

import { isValidCnpj } from "@/lib/validation";

const clientFieldsSchema = z.object({
  company_name: z.string().trim().min(3).max(150).nullable().optional(),
  cnpj: z.string().refine(isValidCnpj, "CNPJ inválido.").nullable().optional(),
  phone: z
    .string()
    .regex(/^\d{10,11}$/, "Telefone inválido.")
    .nullable()
    .optional(),
  email: z.email().max(255).nullable().optional(),
  address: z.string().max(255).nullable().optional(),
  city: z.string().trim().min(2).max(100).nullable().optional(),
  state: z
    .string()
    .regex(/^[A-Z]{2}$/, "Estado inválido.")
    .nullable()
    .optional(),
});

export const createClientSchema = clientFieldsSchema
  .extend({
    client_type: z.enum(["individual", "company"]),
  })
  .superRefine((client, context) => {
    if (client.client_type !== "company") {
      return;
    }

    if (!client.company_name?.trim()) {
      context.addIssue({
        code: "custom",
        path: ["company_name"],
        message: "Nome da empresa requerido.",
      });
    }

    if (!client.cnpj) {
      context.addIssue({
        code: "custom",
        path: ["cnpj"],
        message: "CNPJ requerido para empresas.",
      });
    }
  });

export const updateClientSchema = clientFieldsSchema;
