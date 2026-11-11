import { AlertTriangle, FolderKanban, ListTodo } from "lucide-react";

import { useAuth } from "@/hooks";
import { LoadingPage } from "@/layouts";

import {
  DashboardAlerts,
  DashboardHeader,
  DashboardPanel,
  DashboardProjectList,
  DashboardTaskList,
  StatisticsCards,
} from "../components";

import { useDashboard } from "../hooks";

export function DashboardPage() {
  const { data: response, isLoading, error, refetch } = useDashboard();
  const { session } = useAuth();

  const isClient = session?.user?.role === "client";

  if (isLoading) {
    return <LoadingPage />;
  }

  if (error || !response?.data) {
    return (
      <div className="flex min-h-64 flex-col items-center justify-center text-center">
        <AlertTriangle size={30} className="mb-3 text-muted-foreground" />

        <h2 className="font-semibold">Não foi possível carregar o dashboard</h2>

        <p className="mt-1 text-sm text-muted-foreground">
          Ocorreu um erro ao buscar os dados. Tente novamente.
        </p>

        <button
          type="button"
          onClick={() => void refetch()}
          className="mt-4 rounded-md border border-border px-4 py-2 text-sm font-medium transition hover:bg-muted"
        >
          Tentar novamente
        </button>
      </div>
    );
  }

  const { statistics, projects, tasks } = response.data;

  return (
    <div className="space-y-8">
      <DashboardHeader
        userName={session?.user?.name ?? ""}
        isClient={isClient}
      />

      <StatisticsCards statistics={statistics} isClient={isClient} />

      <section
        className={`grid gap-6 ${
          isClient ? "xl:grid-cols-3" : "xl:grid-cols-2"
        }`}
      >
        <DashboardPanel
          title={isClient ? "Seus projetos" : "Projetos em andamento"}
          description={
            isClient
              ? "Veja o andamento, as etapas concluídas e a previsão de cada projeto."
              : "Acompanhe os projetos ativos vinculados à sua conta."
          }
          icon={FolderKanban}
          className={isClient ? "xl:col-span-2" : undefined}
        >
          <DashboardProjectList projects={projects} isClient={isClient} />
        </DashboardPanel>

        <DashboardPanel
          title={isClient ? "Próximas atividades" : "Minhas tarefas"}
          description={
            isClient
              ? "Veja as próximas atividades dos seus projetos."
              : "Acompanhe as atividades atribuídas a você."
          }
          icon={ListTodo}
        >
          <DashboardTaskList tasks={tasks} />
        </DashboardPanel>
      </section>

      <DashboardAlerts overdueTasks={statistics.overdue_tasks} />
    </div>
  );
}
