export const invitationQueryKeys = {
  all: ["invitations"] as const,
  list: () => [...invitationQueryKeys.all, "list"] as const,
};
