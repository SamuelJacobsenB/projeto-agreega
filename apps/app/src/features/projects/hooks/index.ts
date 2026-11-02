import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import { projectService } from "../project.service";
import type { CreateProjectDto, UpdateProjectDto } from "../types";
import { projectQueryKeys } from "../query-keys";

export function useProjects() {
  return useQuery({
    queryKey: projectQueryKeys.lists(),
    queryFn: projectService.list,
  });
}

export function useProject(id: string) {
  return useQuery({
    queryKey: projectQueryKeys.detail(id),
    queryFn: () => projectService.getById(id),
    enabled: Boolean(id),
  });
}

export function useCreateProject() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (dto: CreateProjectDto) => projectService.create(dto),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: projectQueryKeys.all }),
  });
}

export function useUpdateProject() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, dto }: { id: string; dto: UpdateProjectDto }) =>
      projectService.update(id, dto),
    onSuccess: (_response, { id }) => {
      queryClient.invalidateQueries({ queryKey: projectQueryKeys.lists() });
      queryClient.invalidateQueries({ queryKey: projectQueryKeys.detail(id) });
    },
  });
}

export function useDeleteProject() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: projectService.delete,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: projectQueryKeys.all }),
  });
}