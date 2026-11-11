import { useState } from "react";

type Props = {
  userName: string;
  isClient: boolean;
};

export function DashboardHeader({ userName, isClient }: Props) {
  const [now] = useState(() => new Date());
  const hour = now.getHours();
  const date = new Intl.DateTimeFormat("pt-BR", {
    weekday: "long",
    day: "numeric",
    month: "long",
    year: "numeric",
  }).format(now);
  const greeting =
    hour < 12 ? "Bom dia" : hour < 18 ? "Boa tarde" : "Boa noite";
  const firstName = userName.trim().split(/\s+/)[0];

  return (
    <header className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
      <div>
        <p className="text-sm text-muted-foreground capitalize">{date}</p>

        <h1 className="mt-1 text-2xl font-semibold tracking-tight">
          {greeting}
          {firstName ? `, ${firstName}` : ""}!
        </h1>

        <p className="mt-1 text-sm text-muted-foreground">
          {isClient
            ? "Acompanhe seus projetos e veja as próximas atividades."
            : "Aqui está o resumo dos projetos e tarefas atribuídos a você."}
        </p>
      </div>
    </header>
  );
}
