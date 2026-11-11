import { CheckCircle2, Clock3, FolderKanban } from "lucide-react";

import type { DashboardStatistics, StatisticCard } from "../types";

type Props = {
  statistics: DashboardStatistics;
  isClient: boolean;
};

export function StatisticsCards({ statistics, isClient }: Props) {
  const cards: StatisticCard[] = [
    {
      title: "Projetos ativos",
      value: statistics.active_projects,
      description: isClient
        ? "Em planejamento ou execução"
        : "Em andamento atualmente",
      icon: FolderKanban,
    },
    {
      title: isClient ? "Atividades pendentes" : "Tarefas pendentes",
      value: statistics.pending_tasks,
      description: isClient
        ? "Próximas etapas dos seus projetos"
        : "Aguardando conclusão",
      icon: CheckCircle2,
    },
    {
      title: isClient ? "Atividades atrasadas" : "Tarefas atrasadas",
      value: statistics.overdue_tasks,
      description: isClient
        ? "Atividades que passaram do prazo"
        : "Precisam de atenção",
      icon: Clock3,
      alert: true,
    },
    {
      title: "Projetos concluídos",
      value: statistics.completed_projects,
      description: "Finalizados",
      icon: CheckCircle2,
    },
  ];

  return (
    <section className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {cards.map(({ title, value, description, icon: Icon, alert }) => (
        <div
          key={title}
          className="rounded-xl border border-border bg-card p-5"
        >
          <div className="flex items-center justify-between">
            <span className="text-sm text-muted-foreground">{title}</span>

            <div
              className={`flex size-9 items-center justify-center rounded-lg ${
                alert
                  ? "bg-red-500/10 text-red-500"
                  : "bg-muted text-foreground"
              }`}
            >
              <Icon size={18} />
            </div>
          </div>

          <p className="mt-4 text-3xl font-semibold tracking-tight">{value}</p>

          <p className="mt-1 text-xs text-muted-foreground">{description}</p>
        </div>
      ))}
    </section>
  );
}
