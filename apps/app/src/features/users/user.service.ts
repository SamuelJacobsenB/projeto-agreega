import { api, apiRequest } from "@/api";

import type { UpdateUserDto, User } from "./types";

export const userService = {
  getMine: () => apiRequest<User>(api.get("/users/me")),

  list: () => apiRequest<User[]>(api.get("/users")),

  getById: (id: string) => apiRequest<User>(api.get(`/users/${id}`)),

  update: (id: string, dto: UpdateUserDto) =>
    apiRequest<User>(api.put(`/users/${id}`, dto)),

  delete: (id: string) => apiRequest<null>(api.delete(`/users/${id}`)),
};
