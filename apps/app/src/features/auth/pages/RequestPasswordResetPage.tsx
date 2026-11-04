import { useState, type FormEvent } from "react";
import { Link, useNavigate } from "react-router-dom";

import { Button, Error, Input, Loader } from "@/components";
import { useToast } from "@/hooks";
import { getApiError } from "@/lib";
import type { ApiError } from "@/types";

import { useRequestPasswordReset } from "../hooks";

export function RequestPasswordResetPage() {
  const navigate = useNavigate();

  const { notify } = useToast();

  const { requestPasswordReset, isPending } = useRequestPasswordReset();
  const [submitError, setSubmitError] = useState<ApiError | null>(null);

  const [email, setEmail] = useState("");

  async function handleSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    try {
      const { message } = await requestPasswordReset(email);
      notify.success(message);
      navigate("/redefinir-senha/confirmar", { replace: true });
    } catch (error) {
      setSubmitError(getApiError(error));
    }
  }

  return (
    <>
      <div className="mb-8">
        <h1 className="text-2xl font-semibold tracking-tight text-foreground">
          Solicitar redefinição de senha
        </h1>

        <p className="mt-4 text-sm text-muted-foreground">
          Informe seu e-mail para redefinir sua senha. Um código será enviado
          para o seu e-mail para que você possa criar uma nova senha.
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <Input
            label="E-mail"
            id="email"
            name="email"
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="seu@email.com"
            autoComplete="email"
            error={submitError?.fields?.email}
            required
            disabled={isPending}
          />

          <div className="mt-2 text-right">
            <Link
              to="/redefinir-senha/solicitar"
              className="text-xs text-muted-foreground transition hover:text-foreground"
            >
              Voltar a página de login
            </Link>
          </div>
        </div>

        <Error message={submitError?.message} />

        <Button
          type="submit"
          className="w-full"
          disabled={!email.trim() || isPending}
        >
          {isPending ? <Loader /> : "Solicitar"}
        </Button>
      </form>
    </>
  );
}
