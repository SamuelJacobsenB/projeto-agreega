import type { z } from "zod";

import type {
  acceptInvitationSchema,
  createInvitationSchema,
} from "../schemas";
import type { UserRole } from "@/features/users";

export type Invitation = {
  id: string;
  email: string;
  invited_by: string;
  client_id: string | null;
  role: UserRole;
  expires_at: string;
  accepted_at: string | null;
  created_at: string;
};
export type CreateInvitationDto = z.infer<typeof createInvitationSchema>;
export type AcceptInvitationDto = z.infer<typeof acceptInvitationSchema>;
