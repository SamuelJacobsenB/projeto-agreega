export const clientQueryKeys = {
  all: ["clients"] as const,
  lists: () => [...clientQueryKeys.all, "list"] as const,
  details: () => [...clientQueryKeys.all, "detail"] as const,
  detail: (id: string) => [...clientQueryKeys.details(), id] as const,
  mine: () => [...clientQueryKeys.all, "mine"] as const,
};
