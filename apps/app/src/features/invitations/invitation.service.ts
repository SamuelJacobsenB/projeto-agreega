import { api, apiRequest } from "@/api";

import type {
  AcceptInvitationDto,
  CreateInvitationDto,
  Invitation,
} from "./types";

export const invitationService = {
  list: () => apiRequest<Invitation[]>(api.get("/invitations")),
  create: (dto: CreateInvitationDto) =>
    apiRequest<Invitation>(api.post("/invitations", dto)),
  accept: (dto: AcceptInvitationDto) =>
    apiRequest<null>(api.post("/invitations/accept", dto)),
  delete: (id: string) => apiRequest<null>(api.delete(`/invitations/${id}`)),
};
