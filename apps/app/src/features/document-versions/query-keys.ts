export const documentVersionQueryKeys = {
  all: ["document-versions"] as const,
  byDocument: (documentId: string) =>
    [...documentVersionQueryKeys.all, "document", documentId] as const,
  detail: (id: string) =>
    [...documentVersionQueryKeys.all, "detail", id] as const,
};
