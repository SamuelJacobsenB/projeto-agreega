import type { z } from "zod";

import type { updateUserSchema, userRoleSchema } from "../schemas";

export type UserRole = z.infer<typeof userRoleSchema>;

export type User = {
  id: string;
  name: string;
  email: string;
  phone: string | null;
  cpf: string | null;
  role: UserRole;
  client_id: string | null;
  avatar_file_id: string | null;
  created_at: string;
  updated_at: string;
};

export type UpdateUserDto = z.infer<typeof updateUserSchema>;
