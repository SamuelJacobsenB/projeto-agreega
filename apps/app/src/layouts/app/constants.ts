import {
  FolderKanban,
  LayoutDashboard,
  MessageSquare,
  ShieldCheck,
  Users,
  type LucideIcon,
} from "lucide-react";

export type UserRole =
  "client" | "engineer" | "architect" | "technical" | "admin";

export type NavigationItem = {
  label: string;
  href: string;
  icon: LucideIcon;
  end?: boolean;
};

const dashboard: NavigationItem = {
  label: "Visão geral",
  href: "/dashboard",
  icon: LayoutDashboard,
  end: true,
};

const projects: NavigationItem = {
  label: "Projetos",
  href: "/projetos",
  icon: FolderKanban,
};

const messages: NavigationItem = {
  label: "Mensagens",
  href: "/mensagens",
  icon: MessageSquare,
};

const clients: NavigationItem = {
  label: "Clientes",
  href: "/clientes",
  icon: Users,
};

const users: NavigationItem = {
  label: "Usuários",
  href: "/usuarios",
  icon: Users,
};

const audit: NavigationItem = {
  label: "Auditoria",
  href: "/auditoria",
  icon: ShieldCheck,
};

const staffNavigation: NavigationItem[] = [dashboard, projects, messages];

export const navigation: Record<UserRole, NavigationItem[]> = {
  client: [dashboard, projects, messages],

  engineer: staffNavigation,

  architect: staffNavigation,

  technical: staffNavigation,

  admin: [...staffNavigation, clients, users, audit],
};
