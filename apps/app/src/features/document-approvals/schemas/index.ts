import { z } from "zod";

export const approvalStatusSchema = z.enum([
  "pending",
  "approved",
  "changes_requested",
  "rejected",
]);

export const createDocumentApprovalSchema = z.object({
  document_id: z.uuid(),
  document_version_id: z.uuid(),
  comment: z.string().max(500).nullable().optional(),
});

export const reviewDocumentApprovalSchema = z.object({
  status: approvalStatusSchema,
  comment: z.string().max(500).nullable().optional(),
});
