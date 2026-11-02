export const projectImageQueryKeys = {
  all: ["project-images"] as const,
  byProject: (projectId: string) =>
    [...projectImageQueryKeys.all, "project", projectId] as const,
  covers: () => [...projectImageQueryKeys.all, "covers"] as const,
  cover: (projectId: string) =>
    [...projectImageQueryKeys.all, "cover", projectId] as const,
  detail: (id: string) => [...projectImageQueryKeys.all, "detail", id] as const,
  file: (id: string) =>
    [...projectImageQueryKeys.detail(id), "file"] as const,
};
