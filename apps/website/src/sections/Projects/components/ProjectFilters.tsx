import { projectFilters, type ProjectFilter } from "../constants";

type ProjectFiltersProps = {
  activeFilter: ProjectFilter;
  onChange: (filter: ProjectFilter) => void;
};

export function ProjectFilters({
  activeFilter,
  onChange,
}: ProjectFiltersProps) {
  return (
    <div className="mt-14 flex flex-wrap gap-2 border-b border-gray-200 pb-5 lg:mt-20">
      {projectFilters.map((filter) => {
        const isActive = activeFilter === filter.value;

        return (
          <button
            key={filter.value}
            type="button"
            onClick={() => onChange(filter.value)}
            aria-pressed={isActive}
            className={`inline-flex items-center border px-4 py-2.5 text-xs transition-colors duration-200 ${
              isActive
                ? "border-(--color-light-black) bg-(--color-light-black) text-white"
                : "border-gray-200 text-gray-600 hover:border-(--color-light-black) hover:text-(--color-light-black)"
            }`}
          >
            {filter.label}
          </button>
        );
      })}
    </div>
  );
}
