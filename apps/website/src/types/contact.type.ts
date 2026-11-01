type ContactBase = {
  email: string;
  phone: string;
  city: string;
  state: string;
  address: string;
  subject: string;
  message: string;
};

export type IndividualContactFormData = ContactBase & {
  clientType: "individual";
  name: string;
};

export type CompanyContactFormData = ContactBase & {
  clientType: "company";
  companyName: string;
  cnpj: string;
};

export type ContactFormData =
  | IndividualContactFormData
  | CompanyContactFormData;
