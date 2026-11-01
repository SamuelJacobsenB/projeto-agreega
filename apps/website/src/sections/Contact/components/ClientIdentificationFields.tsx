import type { ChangeEvent } from "react";

import { Input } from "../../../components/ui";
import type { ContactFormData } from "../../../types";

type Props = {
  formData: ContactFormData;
  onChange: (event: ChangeEvent<HTMLInputElement>) => void;
};

export function ClientIdentificationFields({ formData, onChange }: Props) {
  if (formData.clientType === "individual") {
    return (
      <Input
        id="name"
        name="name"
        label="Nome completo"
        placeholder="Seu nome completo"
        autoComplete="name"
        value={formData.name}
        onChange={onChange}
        required
      />
    );
  }

  return (
    <div className="grid gap-7 sm:grid-cols-2">
      <Input
        id="companyName"
        name="companyName"
        label="Razão social"
        placeholder="Nome da empresa"
        autoComplete="organization"
        value={formData.companyName}
        onChange={onChange}
        required
      />

      <Input
        id="cnpj"
        name="cnpj"
        label="CNPJ"
        placeholder="00.000.000/0000-00"
        inputMode="numeric"
        maxLength={14}
        value={formData.cnpj}
        onChange={onChange}
        required
      />
    </div>
  );
}
