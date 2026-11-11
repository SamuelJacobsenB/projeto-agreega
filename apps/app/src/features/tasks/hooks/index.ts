import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { dashboardQueryKeys } from "@/features/dashboard/query-keys";

import { taskService } from "../task.service";
import { taskQueryKeys } from "../query-keys";
import type { CreateTaskDto } from "../types";

export function useStageTasks(stageId: string) {
  return useQuery({
    queryKey: taskQueryKeys.byStage(stageId),
    queryFn: () => taskService.listByStage(stageId),
    enabled: Boolean(stageId),
  });
}

export function useTask(id: string) {
  return useQuery({
    queryKey: taskQueryKeys.detail(id),
    queryFn: () => taskService.getById(id),
    enabled: Boolean(id),
  });
}

export function useCreateTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateTaskDto) => taskService.create(dto),
    onSuccess: (_response, dto) =>
      Promise.all([
        queryClient.invalidateQueries({
          queryKey: taskQueryKeys.byStage(dto.stage_id),
        }),
        queryClient.invalidateQueries({ queryKey: dashboardQueryKeys.all }),
      ]),
  });
}

export function useReorderTasks() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ stageId, tasks }: { stageId: string; tasks: string[] }) =>
      taskService.reorder(stageId, tasks),
    onSuccess: (_response, { stageId }) =>
      queryClient.invalidateQueries({ queryKey: taskQueryKeys.byStage(stageId) }),
  });
}

export function useDeleteTask() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: taskService.delete,
    onSuccess: () =>
      Promise.all([
        queryClient.invalidateQueries({ queryKey: taskQueryKeys.all }),
        queryClient.invalidateQueries({ queryKey: dashboardQueryKeys.all }),
      ]),
  });
}
