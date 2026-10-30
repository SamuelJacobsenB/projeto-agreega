import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "../../lib";
import { projectService } from "../../services";

export function useProjects() {
  return useQuery({
    queryKey: queryKeys.projects.all,
    queryFn: projectService.getAll,
    select: (response) => response.data ?? [],
  });
}
