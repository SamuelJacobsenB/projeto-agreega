export const documentCommentQueryKeys = {
  all: ["document-comments"] as const,
  byVersion: (versionId: string) =>
    [...documentCommentQueryKeys.all, "version", versionId] as const,
};
