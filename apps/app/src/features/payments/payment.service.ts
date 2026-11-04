import { api, apiRequest } from "@/api";

import type { CreatePaymentDto, Payment, UpdatePaymentDto } from "./types";

export const paymentService = {
  listByProject: (projectId: string) =>
    apiRequest<Payment[]>(api.get(`/payments/project/${projectId}`)),

  create: (dto: CreatePaymentDto) =>
    apiRequest<Payment>(api.post("/payments", dto)),

  update: (id: string, dto: UpdatePaymentDto) =>
    apiRequest<Payment>(api.patch(`/payments/${id}`, dto)),
};
