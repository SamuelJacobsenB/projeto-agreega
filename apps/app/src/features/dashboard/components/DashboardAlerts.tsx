import { AlertTriangle } from "lucide-react";

import { DashboardEmptyState } from "./DashboardEmptyState";
import { DashboardPanel } from "./DashboardPanel";

type DashboardAlertsProps = {
  overdueTasks: number;
};

export function DashboardAlerts({ overdueTasks }: DashboardAlertsProps) {
  const hasAlerts = overdueTasks > 0;

  return (
    <DashboardPanel
      title="Atenção necessária"
      description="Pendências que precisam ser verificadas."
    >
      {hasAlerts ? (
        <div className="p-5">
          <div className="flex items-start gap-3 rounded-lg border border-red-500/20 bg-red-500/5 p-3">
            <AlertTriangle
              size={18}
              className="mt-0.5 shrink-0 text-red-500"
              aria-hidden="true"
            />

            <div className="min-w-0">
              <p className="text-sm font-medium">Tarefas atrasadas</p>

              <p className="mt-1 text-xs text-muted-foreground">
                {overdueTasks === 1
                  ? "Existe 1 tarefa que ultrapassou o prazo."
                  : `Existem ${overdueTasks} tarefas que ultrapassaram o prazo.`}
              </p>
            </div>
          </div>
        </div>
      ) : (
        <DashboardEmptyState
          title="Tudo em dia"
          description="Não existem pendências que precisem de atenção neste momento."
        />
      )}
    </DashboardPanel>
  );
}
