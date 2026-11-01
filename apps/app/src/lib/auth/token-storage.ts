export const tokenStorage = {
  set: (accessToken: string, expiresAt: Date) => {
    localStorage.setItem("access_token", accessToken);
    localStorage.setItem("expires_at", expiresAt.toISOString());
  },
  get: () => {
    const accessToken = localStorage.getItem("access_token");
    const expiresAt = localStorage.getItem("expires_at");

    if (!accessToken || !expiresAt) {
      return null;
    }

    return {
      accessToken,
      expiresAt: new Date(expiresAt),
    };
  },
  clear: () => {
    localStorage.removeItem("access_token");
    localStorage.removeItem("expires_at");
  },
};
