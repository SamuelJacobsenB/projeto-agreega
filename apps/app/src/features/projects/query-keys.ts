export const projectQueryKeys = {
  all: ["projects"] as const,
  lists: () => [...projectQueryKeys.all, "list"] as const,
  details: () => [...projectQueryKeys.all, "detail"] as const,
  detail: (id: string) => [...projectQueryKeys.details(), id] as const,
};