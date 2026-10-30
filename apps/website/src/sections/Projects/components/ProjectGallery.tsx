import { ChevronLeft, ChevronRight } from "lucide-react";
import { useState } from "react";

import { Loader } from "../../../components/ui";
import { useProjectImages } from "../../../hooks/project-images";

type ProjectGalleryProps = {
  projectId: string;
  projectName: string;
};

export function ProjectGallery({
  projectId,
  projectName,
}: ProjectGalleryProps) {
  const { data: images = [], isLoading, isError } = useProjectImages(projectId);

  const [selection, setSelection] = useState({
    projectId,
    index: 0,
  });

  const activeIndex = selection.projectId === projectId ? selection.index : 0;

  function selectImage(index: number) {
    setSelection({ projectId, index });
  }

  function handlePrevious() {
    selectImage(activeIndex === 0 ? images.length - 1 : activeIndex - 1);
  }

  function handleNext() {
    selectImage(activeIndex === images.length - 1 ? 0 : activeIndex + 1);
  }

  if (isLoading) {
    return (
      <div className="mt-9 flex min-h-64 items-center justify-center bg-gray-100">
        <Loader />
      </div>
    );
  }

  if (isError || images.length === 0) return null;

  const safeIndex = Math.min(activeIndex, images.length - 1);
  const activeImage = images[safeIndex];

  return (
    <div className="mt-9">
      <div className="relative overflow-hidden bg-gray-100">
        <img
          src={activeImage.url}
          alt={`${projectName} — imagem ${safeIndex + 1}`}
          className="max-h-[68vh] min-h-64 w-full object-cover sm:min-h-96"
        />

        {images.length > 1 && (
          <>
            <button
              type="button"
              onClick={handlePrevious}
              aria-label="Imagem anterior"
              className="absolute left-3 top-1/2 flex h-10 w-10 -translate-y-1/2 items-center justify-center rounded-full bg-white/90 transition hover:bg-white focus-visible:outline-2"
            >
              <ChevronLeft size={20} />
            </button>

            <button
              type="button"
              onClick={handleNext}
              aria-label="Próxima imagem"
              className="absolute right-3 top-1/2 flex h-10 w-10 -translate-y-1/2 items-center justify-center rounded-full bg-white/90 transition hover:bg-white focus-visible:outline-2"
            >
              <ChevronRight size={20} />
            </button>

            <span
              aria-live="polite"
              className="absolute bottom-3 right-3 bg-black/65 px-3 py-1.5 text-xs text-white"
            >
              {safeIndex + 1} / {images.length}
            </span>
          </>
        )}
      </div>

      {images.length > 1 && (
        <div className="mt-3 flex gap-3 overflow-x-auto pb-2">
          {images.map((image, index) => {
            const isActive = safeIndex === index;

            return (
              <button
                key={image.id}
                type="button"
                onClick={() => selectImage(index)}
                aria-label={`Ver imagem ${index + 1}`}
                aria-pressed={isActive}
                className={`h-16 w-24 shrink-0 overflow-hidden border-2 transition-all duration-75 sm:h-20 sm:w-32 ${
                  isActive
                    ? "border-(--color-light-black)"
                    : "border-transparent opacity-65 hover:opacity-100"
                }`}
              >
                <img
                  src={image.url}
                  alt={`${projectName} — imagem ${index + 1}`}
                  loading="lazy"
                  className="h-full w-full object-cover"
                />
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}
