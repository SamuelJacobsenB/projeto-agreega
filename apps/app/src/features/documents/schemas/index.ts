import { z } from "zod";

export const documentTypeSchema = z.enum([
  "contract",
  "project",
  "technical_drawing",
  "report",
  "budget",
  "license",
  "certificate",
  "other",
]);

export const documentStatusSchema = z.enum([
  "draft",
  "under_review",
  "approved",
  "changes_requested",
  "archived",
]);

export const createDocumentSchema = z.object({
  project_id: z.uuid(),
  stage_id: z.uuid(),
  task_id: z.uuid().nullable().optional(),
  name: z.string().trim().min(1).max(200),
  document_type: documentTypeSchema,
  status: documentStatusSchema,
});

export const updateDocumentSchema = createDocumentSchema
  .omit({ project_id: true, stage_id: true, task_id: true })
  .partial();
