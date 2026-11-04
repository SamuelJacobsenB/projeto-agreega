import { useState, type ChangeEvent, type FormEvent } from "react";
import { Link, useNavigate, useSearchParams } from "react-router-dom";

import { Button, Error, Input, Loader } from "@/components";
import { useToast } from "@/hooks";
import { getApiError } from "@/lib";
import type { ApiError } from "@/types";

import { useConfirmPasswordReset } from "../hooks";
import type { ResetPasswordRequestDto } from "../types";

export function ConfirmPasswordResetPage() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();

  const tokenFromUrl = searchParams.get("token") ?? "";

  const { notify } = useToast();

  const { confirmPasswordReset, isPending, error } = useConfirmPasswordReset();
  const [submitError, setSubmitError] = useState<ApiError | null>(null);

  const [dto, setDto] = useState<ResetPasswordRequestDto>({
    token: tokenFromUrl,
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
      const { message } = await confirmPasswordReset(dto);
      notify.success(message);
      navigate("/login", { replace: true });
    } catch (error) {
      setSubmitError(getApiError(error));
    }
  }

  return (
    <>
      <div className="mb-8">
        <h1 className="text-2xl font-semibold tracking-tight text-foreground">
          Confirmar redefinição de senha
        </h1>

        <p className="mt-4 text-sm text-muted-foreground">
          Informe o token recebido por e-mail e a nova senha para redefinir sua
          senha.
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-4">
        <Input
          label="Token"
          id="token"
          name="token"
          type="text"
          value={dto.token}
          onChange={handleFieldChange}
          placeholder="Token recebido por e-mail"
          autoComplete="off"
          error={submitError?.fields?.token}
          required
          disabled={isPending}
        />

        <div>
          <Input
            label="Nova senha"
            id="new-password"
            name="new-password"
            type="password"
            value={dto.password}
            onChange={handleFieldChange}
            placeholder="Sua nova senha"
            autoComplete="new-password"
            error={submitError?.fields?.password}
            required
            disabled={isPending}
          />

          <div className="mt-2 text-right">
            <Link
              to="/redefinir-senha/solicitar"
              className="text-xs text-muted-foreground transition hover:text-foreground"
            >
              Solicitar novo token
            </Link>
          </div>
        </div>

        <Error message={submitError?.message ?? error?.message} />

        <Button
          type="submit"
          disabled={isPending || !dto.token.trim() || !dto.password}
          className="w-full"
        >
          {isPending ? <Loader /> : "Redefinir senha"}
        </Button>
      </form>
    </>
  );
}
