import { Loader } from "../../../components/ui";

export function ProjectsLoading() {
  return (
    <div id="projetos" className="flex min-h-96 items-center justify-center">
      <Loader />
    </div>
  );
}
