import type { Client } from "@/features/clients";
import type { User } from "@/features/users";

export type Session = {
  user: User | null;
  client: Client | null;
};
