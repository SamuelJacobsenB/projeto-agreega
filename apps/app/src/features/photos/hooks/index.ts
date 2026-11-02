import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { photoService } from "../photo.service";
import { photoQueryKeys } from "../query-keys";
import type { CreatePhotoDto } from "../types";

export function useStagePhotos(stageId: string) {
  return useQuery({
    queryKey: photoQueryKeys.byStage(stageId),
    queryFn: () => photoService.listByStage(stageId),
    enabled: Boolean(stageId),
  });
}

export function useTaskPhotos(taskId: string) {
  return useQuery({
    queryKey: photoQueryKeys.byTask(taskId),
    queryFn: () => photoService.listByTask(taskId),
    enabled: Boolean(taskId),
  });
}

export function usePhoto(id: string) {
  return useQuery({
    queryKey: photoQueryKeys.detail(id),
    queryFn: () => photoService.getById(id),
    enabled: Boolean(id),
  });
}

export function usePhotoFile(id: string) {
  return useQuery({
    queryKey: photoQueryKeys.file(id),
    queryFn: () => photoService.getFile(id),
    enabled: Boolean(id),
  });
}

export function useCreatePhoto() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ dto, file }: { dto: CreatePhotoDto; file: File }) =>
      photoService.create(dto, file),
    onSuccess: (_response, { dto }) => {
      queryClient.invalidateQueries({
        queryKey: photoQueryKeys.byStage(dto.stage_id),
      });
      if (dto.task_id) {
        queryClient.invalidateQueries({
          queryKey: photoQueryKeys.byTask(dto.task_id),
        });
      }
    },
  });
}

export function useUpdatePhotoDescription() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, description }: { id: string; description: string }) =>
      photoService.updateDescription(id, description),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: photoQueryKeys.all }),
  });
}

export function useDeletePhoto() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: photoService.delete,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: photoQueryKeys.all }),
  });
}
