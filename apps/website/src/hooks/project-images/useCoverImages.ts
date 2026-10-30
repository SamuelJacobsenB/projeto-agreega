import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "../../lib";
import { projectImageService } from "../../services";

export function useCoverImages() {
  return useQuery({
    queryKey: queryKeys.projectImages.covers,
    queryFn: projectImageService.getCovers,
    select: (response) => response.data ?? [],
  });
}
