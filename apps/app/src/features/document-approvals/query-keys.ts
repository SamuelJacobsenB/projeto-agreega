export const documentApprovalQueryKeys = {
  all: ["document-approvals"] as const,
  byDocument: (documentId: string) =>
    [...documentApprovalQueryKeys.all, "document", documentId] as const,
  detail: (id: string) =>
    [...documentApprovalQueryKeys.all, "detail", id] as const,
};
