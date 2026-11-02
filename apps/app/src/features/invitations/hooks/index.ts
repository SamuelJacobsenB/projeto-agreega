import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { invitationService } from "../invitation.service";
import { invitationQueryKeys } from "../query-keys";
import type { AcceptInvitationDto, CreateInvitationDto } from "../types";

export function useInvitations() {
  return useQuery({
    queryKey: invitationQueryKeys.list(),
    queryFn: invitationService.list,
  });
}

export function useCreateInvitation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateInvitationDto) => invitationService.create(dto),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: invitationQueryKeys.all }),
  });
}

export function useAcceptInvitation() {
  return useMutation({
    mutationFn: (dto: AcceptInvitationDto) => invitationService.accept(dto),
  });
}

export function useDeleteInvitation() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: invitationService.delete,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: invitationQueryKeys.all }),
  });
}
