import type { z } from "zod";

import type {
  approvalStatusSchema,
  createDocumentApprovalSchema,
  reviewDocumentApprovalSchema,
} from "../schemas";

export type ApprovalStatus = z.infer<typeof approvalStatusSchema>;
export type DocumentApproval = {
  id: string;
  document_id: string;
  document_version_id: string;
  requested_by: string;
  reviewed_by: string | null;
  status: ApprovalStatus;
  comment: string | null;
  created_at: string;
  reviewed_at: string | null;
};
export type CreateDocumentApprovalDto = z.infer<
  typeof createDocumentApprovalSchema
>;
export type ReviewDocumentApprovalDto = z.infer<
  typeof reviewDocumentApprovalSchema
>;
