import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "../../lib";
import { projectImageService } from "../../services";

export function useProjectImages(project_id: string) {
  return useQuery({
    queryKey: queryKeys.projectImages.byProject(project_id),
    queryFn: () => projectImageService.getByProjectId(project_id),
    select: (response) => response.data ?? [],
  });
}
