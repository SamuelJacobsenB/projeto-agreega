import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { projectImageService } from "../project-image.service";
import { projectImageQueryKeys } from "../query-keys";
import type { CreateProjectImageDto } from "../types";

export function useProjectImages(projectId: string) {
  return useQuery({
    queryKey: projectImageQueryKeys.byProject(projectId),
    queryFn: () => projectImageService.listByProject(projectId),
    enabled: Boolean(projectId),
  });
}

export function useProjectCoverImages() {
  return useQuery({
    queryKey: projectImageQueryKeys.covers(),
    queryFn: projectImageService.listCovers,
  });
}

export function useProjectCover(projectId: string) {
  return useQuery({
    queryKey: projectImageQueryKeys.cover(projectId),
    queryFn: () => projectImageService.getCoverByProject(projectId),
    enabled: Boolean(projectId),
  });
}

export function useProjectImage(id: string) {
  return useQuery({
    queryKey: projectImageQueryKeys.detail(id),
    queryFn: () => projectImageService.getById(id),
    enabled: Boolean(id),
  });
}

export function useProjectImageFile(id: string) {
  return useQuery({
    queryKey: projectImageQueryKeys.file(id),
    queryFn: () => projectImageService.getFile(id),
    enabled: Boolean(id),
  });
}

export function useCreateProjectImage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ dto, file }: { dto: CreateProjectImageDto; file: File }) =>
      projectImageService.create(dto, file),
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: projectImageQueryKeys.all }),
  });
}

export function useReorderProjectImages() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({
      projectId,
      images,
    }: {
      projectId: string;
      images: string[];
    }) => projectImageService.reorder(projectId, images),
    onSuccess: (_response, { projectId }) => {
      queryClient.invalidateQueries({
        queryKey: projectImageQueryKeys.byProject(projectId),
      });
      queryClient.invalidateQueries({
        queryKey: projectImageQueryKeys.covers(),
      });
      queryClient.invalidateQueries({
        queryKey: projectImageQueryKeys.cover(projectId),
      });
    },
  });
}

export function useDeleteProjectImage() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: projectImageService.delete,
    onSuccess: () =>
      queryClient.invalidateQueries({ queryKey: projectImageQueryKeys.all }),
  });
}
