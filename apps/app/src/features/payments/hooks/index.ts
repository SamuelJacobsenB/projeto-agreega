import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { paymentService } from "../payment.service";
import { paymentQueryKeys } from "../query-keys";
import type { CreatePaymentDto, UpdatePaymentDto } from "../types";

export function useProjectPayments(projectId: string) {
  return useQuery({
    queryKey: paymentQueryKeys.byProject(projectId),
    queryFn: () => paymentService.listByProject(projectId),
    enabled: Boolean(projectId),
  });
}

export function useCreatePayment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreatePaymentDto) => paymentService.create(dto),
    onSuccess: (_response, dto) =>
      queryClient.invalidateQueries({
        queryKey: paymentQueryKeys.byProject(dto.project_id),
      }),
  });
}

export function useUpdatePayment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, dto }: { id: string; dto: UpdatePaymentDto }) =>
      paymentService.update(id, dto),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: paymentQueryKeys.all }),
  });
}
