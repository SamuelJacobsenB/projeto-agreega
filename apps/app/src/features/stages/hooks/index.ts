import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { stageService } from "../stage.service";
import { stageQueryKeys } from "../query-keys";
import type { CreateStageDto } from "../types";

export function useProjectStages(projectId: string) {
  return useQuery({
    queryKey: stageQueryKeys.byProject(projectId),
    queryFn: () => stageService.listByProject(projectId),
    enabled: Boolean(projectId),
  });
}

export function useStage(id: string) {
  return useQuery({
    queryKey: stageQueryKeys.detail(id),
    queryFn: () => stageService.getById(id),
    enabled: Boolean(id),
  });
}

export function useCreateStage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateStageDto) => stageService.create(dto),
    onSuccess: (_response, dto) =>
      queryClient.invalidateQueries({
        queryKey: stageQueryKeys.byProject(dto.project_id),
      }),
  });
}

export function useReorderStages() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, stages }: { projectId: string; stages: string[] }) =>
      stageService.reorder(projectId, stages),
    onSuccess: (_response, { projectId }) =>
      queryClient.invalidateQueries({
        queryKey: stageQueryKeys.byProject(projectId),
      }),
  });
}

export function useDeleteStage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: stageService.delete,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: stageQueryKeys.all }),
  });
}
