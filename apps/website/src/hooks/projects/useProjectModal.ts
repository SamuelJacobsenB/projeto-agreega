import { useCallback, useMemo, useState } from "react";

import type { Project, ProjectImage } from "../../types";

export function useProjectModal(images: ProjectImage[]) {
  const [selectedProject, setSelectedProject] = useState<Project | null>(null);
  const [activeImageIndex, setActiveImageIndex] = useState(0);

  const selectedImages = useMemo(() => {
    if (!selectedProject) return [];

    return images
      .filter((image) => image.project_id === selectedProject.id)
      .sort((a, b) => a.order - b.order);
  }, [images, selectedProject]);

  const openProject = useCallback((project: Project) => {
    setSelectedProject(project);
    setActiveImageIndex(0);
  }, []);

  const closeProject = useCallback(() => {
    setSelectedProject(null);
    setActiveImageIndex(0);
  }, []);

  const selectImage = useCallback(
    (index: number) => {
      if (index < 0 || index >= selectedImages.length) return;
      setActiveImageIndex(index);
    },
    [selectedImages.length],
  );

  const showPreviousImage = useCallback(() => {
    setActiveImageIndex((currentIndex) => {
      if (selectedImages.length === 0) return 0;

      return currentIndex === 0 ? selectedImages.length - 1 : currentIndex - 1;
    });
  }, [selectedImages.length]);

  const showNextImage = useCallback(() => {
    setActiveImageIndex((currentIndex) => {
      if (selectedImages.length === 0) return 0;

      return currentIndex >= selectedImages.length - 1 ? 0 : currentIndex + 1;
    });
  }, [selectedImages.length]);

  return {
    selectedProject,
    selectedImages,
    activeImageIndex,
    openProject,
    closeProject,
    selectImage,
    showPreviousImage,
    showNextImage,
  };
}
