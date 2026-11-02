import type { z } from "zod";

import type { createClientSchema, updateClientSchema } from "../schemas";

export type ClientType = "individual" | "company";

export type Client = {
  id: string;
  client_type: ClientType;
  company_name: string | null;
  cnpj: string | null;
  phone: string | null;
  email: string | null;
  address: string | null;
  city: string | null;
  state: string | null;
  created_at: string;
  updated_at: string;
};

export type CreateClientDto = z.infer<typeof createClientSchema>;
export type UpdateClientDto = z.infer<typeof updateClientSchema>;
