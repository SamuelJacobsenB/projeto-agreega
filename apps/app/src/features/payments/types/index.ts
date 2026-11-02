import type { z } from "zod";

import type {
  createPaymentSchema,
  paymentStatusSchema,
  updatePaymentSchema,
} from "../schemas";

export type PaymentStatus = z.infer<typeof paymentStatusSchema>;
export type Payment = {
  id: string;
  project_id: string;
  description: string;
  amount: number;
  due_date: string | null;
  paid_at: string | null;
  status: PaymentStatus;
  created_at: string;
  updated_at: string;
};
export type CreatePaymentDto = z.infer<typeof createPaymentSchema>;
export type UpdatePaymentDto = z.infer<typeof updatePaymentSchema>;
