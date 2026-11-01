import { useState } from "react";

import { ArrowUpRight } from "lucide-react";

import { Input, Select, TextArea } from "../../../components/ui";
import type { ClientType, ContactFormData } from "../../../types";
import { validateContactForm } from "../../../utils/validation";

import { contactInfo, initialFormData, subjectOptions } from "../constants";

import { whatsappMessage } from "../utils";
import { ClientIdentificationFields } from "./ClientIdentificationFields";
import { ClientTypeSelector } from "./ClientTypeSelector";

type Props = {
  clientType: ClientType;
  onClientTypeChange: (value: ClientType) => void;
};

export function ContactForm({ clientType, onClientTypeChange }: Props) {
  const [formData, setFormData] = useState<ContactFormData>(initialFormData);
  const [error, setError] = useState<string | null>(null);

  function handleClientTypeChange(value: ClientType) {
    onClientTypeChange(value);

    setFormData((previous) => {
      const commonFields = {
        email: previous.email,
        phone: previous.phone,
        city: previous.city,
        state: previous.state,
        address: previous.address,
        subject: previous.subject,
        message: previous.message,
      };

      if (value === "individual") {
        return {
          ...commonFields,
          clientType: "individual",
          name: "",
        };
      }

      return {
        ...commonFields,
        clientType: "company",
        companyName: "",
        cnpj: "",
      };
    });

    setError(null);
  }

  function handleChange(
    event: React.ChangeEvent<
      HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement
    >,
  ) {
    const { name, value } = event.target;

    setFormData((previous) => ({
      ...previous,
      [name]: value,
    }));

    if (error) setError(null);
  }

  function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    setError(null);

    const errorMessage = validateContactForm(formData);

    if (errorMessage) {
      setError(errorMessage);
      return;
    }

    const message = whatsappMessage(formData);

    const whatsappUrl = new URL(`https://wa.me/${contactInfo.phone}`);
    whatsappUrl.searchParams.set("text", message);

    window.open(whatsappUrl.toString(), "_blank", "noopener,noreferrer");
  }

  return (
    <div>
      <h3 className="mb-8 text-xl font-medium">Envie uma mensagem</h3>

      <form onSubmit={handleSubmit} className="space-y-8">
        <ClientTypeSelector
          value={clientType}
          onChange={handleClientTypeChange}
        />

        <ClientIdentificationFields
          formData={formData}
          onChange={handleChange}
        />

        <div className="grid gap-7 sm:grid-cols-2">
          <Input
            id="email"
            name="email"
            type="email"
            label="E-mail"
            placeholder="seu@email.com"
            autoComplete="email"
            value={formData.email}
            onChange={handleChange}
            required
          />

          <Input
            id="phone"
            name="phone"
            type="tel"
            label="Telefone"
            placeholder="(00) 00000-0000"
            autoComplete="tel"
            value={formData.phone}
            onChange={handleChange}
            required
          />
        </div>

        <div className="grid gap-7 sm:grid-cols-2">
          <Input
            id="city"
            name="city"
            label="Cidade"
            placeholder="Sua cidade"
            autoComplete="address-level2"
            value={formData.city}
            onChange={handleChange}
            required
          />

          <Input
            id="state"
            name="state"
            label="Estado"
            placeholder="UF"
            maxLength={2}
            autoComplete="address-level1"
            value={formData.state}
            onChange={handleChange}
            required
          />
        </div>

        <Input
          id="address"
          name="address"
          label="Endereço"
          placeholder="Rua, número e complemento"
          autoComplete="street-address"
          value={formData.address}
          onChange={handleChange}
        />

        <Select
          id="subject"
          name="subject"
          label="Assunto"
          placeholder="Selecione um assunto"
          options={subjectOptions}
          value={formData.subject}
          onChange={handleChange}
          required
        />

        <TextArea
          id="message"
          name="message"
          label="Mensagem"
          rows={4}
          placeholder="Conte um pouco sobre o que você precisa..."
          value={formData.message}
          onChange={handleChange}
          required
        />

        {error && (
          <p role="alert" className="text-sm text-red-500">
            {error}
          </p>
        )}

        <button
          type="submit"
          className="group flex items-center gap-4 bg-white px-7 py-4 text-sm font-medium text-(--color-light-black) transition hover:bg-white/85"
        >
          Enviar mensagem
          <ArrowUpRight
            size={17}
            className="transition-transform group-hover:translate-x-1 group-hover:-translate-y-1"
          />
        </button>
      </form>
    </div>
  );
}
