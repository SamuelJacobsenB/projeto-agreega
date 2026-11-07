import { useCallback, useEffect, useState } from "react";

import { api, apiRequest } from "@/api";
import { AuthContext } from "@/contexts";
import type { Client } from "@/features/clients";
import type { User } from "@/features/users";

import type { Session } from "../types";

type Props = {
  children: React.ReactNode;
};

export function AuthProvider({ children }: Props) {
  const [session, setSession] = useState<Session | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const refreshSession = useCallback(async () => {
    try {
      const { data: user } = await apiRequest<User>(api.get("/users/me"));

      if (!user) {
        throw new Error("Usuário não encontrado.");
      }

      let client: Client | null = null;

      if (user.client_id) {
        const { data } = await apiRequest<Client>(api.get("/clients/me"));

        if (!data) {
          throw new Error("Cliente não encontrado.");
        }

        client = data;
      }

      setSession({ user, client });
    } catch {
      setSession(null);
    }
  }, []);

  useEffect(() => {
    async function initialize() {
      try {
        await refreshSession();
      } finally {
        setIsLoading(false);
      }
    }

    initialize();
  }, [refreshSession]);

  const value = {
    session,
    isAuthenticated: session !== null,
    isLoading,
    refreshSession,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
