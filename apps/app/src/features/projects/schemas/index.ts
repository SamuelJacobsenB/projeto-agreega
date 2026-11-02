import { z } from "zod";

export const projectTypeSchema = z.enum([
  "architecture",
  "engineering",
  "architecture_and_engineering",
  "consulting",
  "other",
]);

export const projectStatusSchema = z.enum([
  "planning",
  "in_progress",
  "paused",
  "completed",
  "cancelled",
]);

export const createProjectSchema = z.object({
  client_id: z.uuid(),
  name: z.string().trim().min(1),
  description: z.string().max(4000).nullable().optional(),
  project_type: projectTypeSchema,
  status: projectStatusSchema,
  address: z.string().nullable().optional(),
  city: z.string().trim().min(1),
  state: z.string().trim().min(1),
  start_date: z.iso.date().nullable().optional(),
  estimated_end_date: z.iso.date().nullable().optional(),
  cover_file_id: z.uuid().nullable().optional(),
});

export const updateProjectSchema = createProjectSchema
  .omit({ client_id: true })
  .partial();