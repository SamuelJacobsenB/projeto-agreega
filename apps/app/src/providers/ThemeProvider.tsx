import {
  useCallback,
  useEffect,
  useLayoutEffect,
  useState,
  type ReactNode,
} from "react";

import { ThemeContext, type ResolvedTheme, type Theme } from "@/contexts";

const STORAGE_KEY = "agreega-theme";
const SYSTEM_THEME_QUERY = "(prefers-color-scheme: dark)";

type Props = {
  children: ReactNode;
};

function isTheme(value: unknown): value is Theme {
  return value === "light" || value === "dark" || value === "system";
}

function getStoredTheme(): Theme {
  try {
    const storedTheme = localStorage.getItem(STORAGE_KEY);

    return isTheme(storedTheme) ? storedTheme : "system";
  } catch {
    return "system";
  }
}

function getSystemTheme(): ResolvedTheme {
  if (typeof window === "undefined" || !window.matchMedia) {
    return "light";
  }

  return window.matchMedia(SYSTEM_THEME_QUERY).matches ? "dark" : "light";
}

export function ThemeProvider({ children }: Props) {
  const [theme, setThemeState] = useState<Theme>(getStoredTheme);
  const [systemTheme, setSystemTheme] = useState<ResolvedTheme>(getSystemTheme);

  const resolvedTheme: ResolvedTheme = theme === "system" ? systemTheme : theme;

  const setTheme = useCallback((newTheme: Theme) => {
    setThemeState(newTheme);
  }, []);

  useEffect(() => {
    if (typeof window === "undefined" || !window.matchMedia) {
      return;
    }

    const mediaQuery = window.matchMedia(SYSTEM_THEME_QUERY);

    function handleChange(event: MediaQueryListEvent) {
      setSystemTheme(event.matches ? "dark" : "light");
    }

    setSystemTheme(mediaQuery.matches ? "dark" : "light");

    mediaQuery.addEventListener("change", handleChange);

    return () => {
      mediaQuery.removeEventListener("change", handleChange);
    };
  }, []);

  useEffect(() => {
    function handleStorage(event: StorageEvent) {
      if (event.key !== STORAGE_KEY) return;

      setThemeState(isTheme(event.newValue) ? event.newValue : "system");
    }

    window.addEventListener("storage", handleStorage);

    return () => {
      window.removeEventListener("storage", handleStorage);
    };
  }, []);

  useLayoutEffect(() => {
    document.documentElement.dataset.theme = resolvedTheme;
    document.documentElement.style.colorScheme = resolvedTheme;

    localStorage.setItem(STORAGE_KEY, theme);
  }, [theme, resolvedTheme]);

  return (
    <ThemeContext.Provider
      value={{
        theme,
        resolvedTheme,
        setTheme,
      }}
    >
      {children}
    </ThemeContext.Provider>
  );
}
