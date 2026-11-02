import { z } from "zod";

export const paymentStatusSchema = z.enum([
  "pending",
  "paid",
  "overdue",
  "cancelled",
]);

export const createPaymentSchema = z.object({
  project_id: z.uuid(),
  description: z.string().trim().min(1).max(255),
  amount: z.number().positive().multipleOf(0.01),
  due_date: z.iso.date().nullable().optional(),
});

export const updatePaymentSchema = z.object({
  description: z.string().trim().min(1).max(255).nullable().optional(),
  amount: z.number().positive().multipleOf(0.01).nullable().optional(),
  due_date: z.iso.date().nullable().optional(),
  status: paymentStatusSchema.optional(),
});
