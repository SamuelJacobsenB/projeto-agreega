import react, { reactCompilerPreset } from "@vitejs/plugin-react";
import { defineConfig } from "vite";

import tailwindcss from "@tailwindcss/vite";

import babel from "@rolldown/plugin-babel";

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
    babel({ presets: [reactCompilerPreset()] }),
  ],
  resolve: {
    alias: {
      "@": import.meta.dirname + "/src",
    },
  },
  server: {
    port: 5174,
  },
});
