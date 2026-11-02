export const notificationQueryKeys = {
  all: ["notifications"] as const,
  list: (params: { limit?: number; offset?: number } = {}) =>
    [...notificationQueryKeys.all, params] as const,
};
