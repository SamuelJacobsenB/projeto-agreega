import { api, apiRequest } from "@/api";

import type { Client, CreateClientDto, UpdateClientDto } from "./types";

export const clientService = {
  getMine: () => apiRequest<Client>(api.get("/clients/me")),

  list: () => apiRequest<Client[]>(api.get("/clients")),

  getById: (id: string) => apiRequest<Client>(api.get(`/clients/${id}`)),

  create: (dto: CreateClientDto) =>
    apiRequest<Client>(api.post("/clients", dto)),

  update: (id: string, dto: UpdateClientDto) =>
    apiRequest<Client>(api.put(`/clients/${id}`, dto)),

  delete: (id: string) => apiRequest<null>(api.delete(`/clients/${id}`)),
};
