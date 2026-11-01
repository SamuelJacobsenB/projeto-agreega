import type { ContactFormData } from "../../types";

export function validateContactForm(data: ContactFormData): string | undefined {
  const commonFields = [
    data.email,
    data.phone,
    data.city,
    data.state,
    data.subject,
    data.message,
  ];

  const identificationField =
    data.clientType === "individual" ? data.name : data.companyName;

  if (
    [
      ...commonFields,
      identificationField,
      ...(data.clientType === "company" ? [data.cnpj] : []),
    ].some((value) => !value.trim())
  ) {
    return "Preecha todos os campos que são obrigatórios.";
  }

  if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(data.email)) {
    return "Formato de e-mail inválido.";
  }

  if (!/^[a-zA-Z]{2}$/.test(data.state)) {
    return "Formato de estado inválido.";
  }
}
