import { CalendarDays } from "lucide-react";

import type { Project } from "../../../types";
import { formatProjectDate, getStatusInformation } from "../utils";

type InformationRowProps = {
  label: string;
  value: string;
};

function InformationRow({ label, value }: InformationRowProps) {
  return (
    <div className="flex gap-4 py-4">
      <CalendarDays
        size={17}
        className="mt-0.5 shrink-0 text-gray-400"
        strokeWidth={1.5}
      />

      <div>
        <dt className="text-xs text-gray-500">{label}</dt>
        <dd className="mt-1 text-sm">{value}</dd>
      </div>
    </div>
  );
}

type ProjectInformationProps = {
  project: Project;
};

export function ProjectInformation({ project }: ProjectInformationProps) {
  const status = getStatusInformation(project.status);

  return (
    <aside className="h-fit border-t border-gray-200 pt-5 lg:border-l lg:border-t-0 lg:pl-7 lg:pt-0">
      <h3 className="text-xs font-medium uppercase tracking-[0.18em] text-gray-500">
        Informações
      </h3>

      <dl className="mt-5 divide-y divide-gray-200">
        <InformationRow
          label="Início"
          value={formatProjectDate(project.start_date) ?? "Não informado"}
        />

        <InformationRow
          label={
            project.status === "completed"
              ? "Conclusão prevista"
              : "Previsão de conclusão"
          }
          value={
            formatProjectDate(project.estimated_end_date) ?? "Não informada"
          }
        />

        {project.completed_at && (
          <InformationRow
            label="Concluído em"
            value={
              formatProjectDate(project.completed_at) ?? "Data não informada"
            }
          />
        )}

        <div className="flex gap-4 py-4">
          <span
            aria-hidden="true"
            className={`mt-1.5 h-2 w-2 shrink-0 rounded-full ${status.color}`}
          />

          <div>
            <dt className="text-xs text-gray-500">Situação</dt>
            <dd className="mt-1 text-sm">{status.label}</dd>
          </div>
        </div>
      </dl>
    </aside>
  );
}
