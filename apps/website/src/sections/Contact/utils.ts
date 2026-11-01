import type { ContactFormData } from "../../types";

function whatsappMessage(formData: ContactFormData) {
  const identification =
    formData.clientType === "individual"
      ? `*Nome:* ${formData.name}`
      : `*Razão social:* ${formData.companyName}\n*CNPJ:* ${formData.cnpj}`;

  return `
*SOLICITAÇÃO DE CONTATO*

*Tipo de cliente:* ${formData.clientType === "individual" ? "Pessoa física" : "Pessoa jurídica"}

${identification}

*E-mail:* ${formData.email}
*Telefone:* ${formData.phone}

*Localização:*
Cidade: ${formData.city}
Estado: ${formData.state}
Endereço: ${formData.address || "Não informado"}

*Assunto:* ${formData.subject}

*Mensagem:* ${formData.message}
    `.trim();
}

function formatPhoneNumber(phone: string) {
  const digits = phone.replace(/\D/g, "");

  if (digits.length === 13) {
    return `+${digits.slice(0, 2)} (${digits.slice(2, 4)}) ${digits.slice(4, 9)}-${digits.slice(9)}`;
  }

  if (digits.length === 12) {
    return `+${digits.slice(0, 2)} (${digits.slice(2, 4)}) ${digits.slice(4, 8)}-${digits.slice(8)}`;
  }

  return phone;
}

export { formatPhoneNumber, whatsappMessage };
