import type { ContactFormData } from "../../types";

const clientTypes = [
  { value: "individual", label: "Pessoa física" },
  { value: "company", label: "Pessoa jurídica" },
] as const;

const subjectOptions = [
  { label: "Arquitetura", value: "architecture" },
  { label: "Engenharia", value: "engineering" },
  { label: "Consultoria", value: "consulting" },
  { label: "Outros assuntos", value: "other" },
];

const contactInfo = {
  email: import.meta.env.VITE_CONTACT_EMAIL || "",
  phone: import.meta.env.VITE_PHONE_NUMBER || "",
  city: import.meta.env.VITE_CONTACT_CITY || "São Gabriel da Palha — ES",
};

const initialFormData: ContactFormData = {
  clientType: "individual",
  name: "",
  email: "",
  phone: "",
  city: "",
  state: "",
  address: "",
  subject: "",
  message: "",
};

export { clientTypes, contactInfo, initialFormData, subjectOptions };
