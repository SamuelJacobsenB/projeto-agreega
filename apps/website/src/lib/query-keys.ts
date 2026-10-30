export const queryKeys = {
  projects: {
    all: ["projects"] as const,
  },

  projectImages: {
    byProject: (projectId: string) =>
      ["project-images", "by-project", projectId] as const,
    covers: ["project-images", "covers"] as const,
  },
};
