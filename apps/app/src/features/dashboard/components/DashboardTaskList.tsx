import { AlertTriangle, CalendarDays } from "lucide-react";

import type { DashboardTask } from "../types";
import { DashboardEmptyState } from "./DashboardEmptyState";

type Props = {
  tasks: DashboardTask[];
};

const taskStatusLabels: Record<DashboardTask["status"], string> = {
  pending: "Pendente",
  in_progress: "Em andamento",
  blocked: "Bloqueada",
};

function formatDate(value: string) {
  return new Intl.DateTimeFormat("pt-BR", {
    day: "numeric",
    month: "short",
    year: "numeric",
    timeZone: "UTC",
  }).format(new Date(value));
}

export function DashboardTaskList({ tasks }: Props) {
  if (tasks.length === 0) {
    return (
      <DashboardEmptyState
        title="Nenhuma tarefa pendente"
        description="As próximas atividades dos seus projetos aparecerão aqui."
      />
    );
  }

  return (
    <ul className="divide-y divide-border">
      {tasks.map((task) => {
        return (
          <li
            key={task.id}
            className="flex items-start justify-between gap-4 px-5 py-4"
          >
            <div className="min-w-0">
              <p className="truncate text-sm font-medium">{task.title}</p>
              <p className="mt-1 truncate text-xs text-muted-foreground">
                {task.project_name} <span aria-hidden="true">·</span>{" "}
                {task.stage_name}
              </p>
            </div>

            <div className="shrink-0 text-right">
              <span
                className={`inline-flex items-center gap-1 rounded-full px-2.5 py-1 text-xs ${
                  task.is_overdue
                    ? "bg-red-500/10 text-red-600"
                    : task.status === "blocked"
                      ? "bg-amber-500/10 text-amber-700"
                      : "bg-muted text-muted-foreground"
                }`}
              >
                {task.is_overdue && (
                  <AlertTriangle size={13} aria-hidden="true" />
                )}
                {task.is_overdue ? "Atrasada" : taskStatusLabels[task.status]}
              </span>
              <p className="mt-1 flex items-center justify-end gap-1 text-xs text-muted-foreground">
                <CalendarDays size={13} aria-hidden="true" />
                {task.due_date ? formatDate(task.due_date) : "Sem prazo"}
              </p>
            </div>
          </li>
        );
      })}
    </ul>
  );
}
