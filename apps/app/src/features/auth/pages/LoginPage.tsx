import { useState, type ChangeEvent, type FormEvent } from "react";
import { Link, useNavigate } from "react-router-dom";

import { Button, Error, Input, Loader } from "@/components";
import { useToast } from "@/hooks";
import { getApiError } from "@/lib";
import type { ApiError } from "@/types";

import { useLogin } from "../hooks";
import type { LoginRequestDto } from "../types";

export function LoginPage() {
  const navigate = useNavigate();

  const { notify } = useToast();

  const { login, isPending, error } = useLogin();
  const [submitError, setSubmitError] = useState<ApiError | null>(null);

  const [dto, setDto] = useState<LoginRequestDto>({
    email: "",
    password: "",
  });

  function handleFieldChange(e: ChangeEvent<HTMLInputElement>) {
    const { name, value } = e.target;

    setDto((prev) => ({
      ...prev,
      [name]: value,
    }));
  }

  async function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    try {
      const { message } = await login(dto);
      notify.success(message);
      navigate("/dashboard", { replace: true });
    } catch (error) {
      setSubmitError(getApiError(error));
    }
  }

  return (
    <>
      <div className="mb-8">
        <h1 className="text-2xl font-semibold tracking-tight text-foreground">
          Entrar
        </h1>

        <p className="mt-4 text-sm text-muted-foreground">
          Informe suas credenciais para continuar.
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        <Input
          label="E-mail"
          id="email"
          name="email"
          type="email"
          value={dto.email}
          onChange={handleFieldChange}
          placeholder="seu@email.com"
          autoComplete="email"
          error={submitError?.fields?.email}
          required
          disabled={isPending}
        />

        <div>
          <Input
            label="Senha"
            id="password"
            name="password"
            type="password"
            value={dto.password}
            onChange={handleFieldChange}
            placeholder="Sua senha"
            autoComplete="current-password"
            error={submitError?.fields?.password}
            required
            disabled={isPending}
          />

          <div className="mt-2 text-right">
            <Link
              to="/redefinir-senha/solicitar"
              className="text-xs text-muted-foreground transition hover:text-foreground"
            >
              Esqueceu sua senha?
            </Link>
          </div>
        </div>

        <Error message={submitError?.message ?? error?.message} />

        <Button
          type="submit"
          disabled={isPending || !dto.email.trim() || !dto.password}
          className="w-full"
        >
          {isPending ? <Loader /> : "Entrar"}
        </Button>
      </form>
    </>
  );
}
