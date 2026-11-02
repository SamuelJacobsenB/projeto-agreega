export const messageQueryKeys = {
  all: ["messages"] as const,
  byProject: (projectId: string) =>
    [...messageQueryKeys.all, "project", projectId] as const,
};
