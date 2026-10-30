import { ArrowDownRight } from "lucide-react";
import { useMemo, useState } from "react";

import { useProjectModal } from "../../hooks/projects";
import type { Project, ProjectImage } from "../../types";
import { mockCoverImages, mockGalleryImages, mockProjects } from "./mocks";

import { ProjectCard, ProjectFilters, ProjectModal } from "./components";
import { type ProjectFilter } from "./constants";
import { organizeProjects } from "./utils";

type ProjectsProps = {
  projects?: Project[];
  projectImages?: ProjectImage[];
  coverImages?: ProjectImage[];
};

export function Projects({
  projects = mockProjects,
  projectImages,
  coverImages = mockCoverImages,
}: ProjectsProps) {
  const [activeFilter, setActiveFilter] = useState<ProjectFilter>("all");

  const allProjectImages = useMemo(() => {
    if (projectImages) return projectImages;

    if (projects === mockProjects) {
      return [...coverImages, ...mockGalleryImages];
    }

    return coverImages;
  }, [projectImages, projects, coverImages]);

  const organizedProjects = useMemo(
    () => organizeProjects(projects, allProjectImages),
    [projects, allProjectImages],
  );

  const filteredProjects = useMemo(
    () =>
      organizedProjects.filter(({ project }) => {
        if (project.status === "cancelled") return false;

        return activeFilter === "all" || project.project_type === activeFilter;
      }),
    [organizedProjects, activeFilter],
  );

  const {
    selectedProject,
    selectedImages,
    activeImageIndex,
    openProject,
    closeProject,
    selectImage,
    showPreviousImage,
    showNextImage,
  } = useProjectModal(allProjectImages);

  const [featuredProject, ...otherProjects] = filteredProjects;

  return (
    <section
      id="projetos"
      className="bg-white px-6 py-24 text-(--color-light-black) sm:px-12 lg:px-24 lg:py-32"
    >
      <div className="mx-auto">
        <header className="grid items-end gap-8 lg:grid-cols-2 lg:gap-20">
          <div>
            <span className="mb-6 block text-sm font-medium uppercase tracking-[0.2em] text-gray-500">
              Nosso portfólio
            </span>

            <h2 className="max-w-xl text-4xl font-light leading-tight sm:text-4xl lg:text-5xl">
              Ideias que ganham forma.
              <span className="mt-2 block font-medium">
                Projetos com propósito.
              </span>
            </h2>
          </div>

          <div className="lg:pb-1">
            <p className="max-w-lg text-base leading-8 text-gray-600">
              Cada projeto representa uma maneira de transformar planejamento,
              técnica e criatividade em espaços e soluções concretas,
              considerando as necessidades de cada cliente.
            </p>

            <a
              href="#contato"
              className="mt-6 inline-flex items-center gap-3 border-b border-(--color-light-black)/30 pb-2 text-sm transition-colors hover:border-(--color-light-black)"
            >
              Vamos construir algo juntos?
              <ArrowDownRight size={16} strokeWidth={1.5} />
            </a>
          </div>
        </header>

        <ProjectFilters
          activeFilter={activeFilter}
          onChange={setActiveFilter}
        />

        {featuredProject ? (
          <div className="mt-8 space-y-5">
            <ProjectCard
              project={featuredProject.project}
              cover_url={featuredProject.cover_url}
              featured
              onSelect={openProject}
            />

            {otherProjects.length > 0 && (
              <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
                {otherProjects.map(({ project, cover_url }) => (
                  <ProjectCard
                    key={project.id}
                    project={project}
                    cover_url={cover_url}
                    onSelect={openProject}
                  />
                ))}
              </div>
            )}
          </div>
        ) : (
          <div className="mt-8 flex min-h-64 items-center justify-center px-6 py-36 text-center">
            <div>
              <p className="text-lg font-medium">
                Nenhum projeto encontrado nesta categoria.
              </p>

              <button
                type="button"
                onClick={() => setActiveFilter("all")}
                className="mt-4 border-b border-gray-400 pb-1 text-sm text-gray-600 transition-colors hover:text-(--color-light-black)"
              >
                Ver todos os projetos
              </button>
            </div>
          </div>
        )}

        <footer className="mt-8 flex flex-col gap-4 border-t border-gray-200 pt-6 sm:flex-row sm:items-center sm:justify-between">
          <p className="text-xs text-gray-500">
            {filteredProjects.length}{" "}
            {filteredProjects.length === 1
              ? "projeto listado"
              : "projetos listados"}
          </p>
        </footer>
      </div>

      <ProjectModal
        project={selectedProject}
        images={selectedImages}
        activeImageIndex={activeImageIndex}
        onClose={closeProject}
        onSelectImage={selectImage}
        onPreviousImage={showPreviousImage}
        onNextImage={showNextImage}
      />
    </section>
  );
}

export default Projects;
