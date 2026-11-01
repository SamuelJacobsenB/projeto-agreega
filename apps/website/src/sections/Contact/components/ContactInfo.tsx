import {
  ArrowUpRight,
  Clock3,
  Mail,
  MapPin,
  MessageCircle,
  Phone,
} from "lucide-react";

import { contactInfo } from "../constants";

import { formatPhoneNumber } from "../utils";
import { ContactInfoItem } from "./ContactInfoItem";

export function ContactInfo() {
  const whatsappNumber = contactInfo.phone;
  const formattedPhone = formatPhoneNumber(whatsappNumber);

  return (
    <aside className="lg:border-l lg:border-white/15 lg:pl-12">
      <h3 className="mb-8 text-xl font-medium">Informações de contato</h3>

      <div className="space-y-9">
        <ContactInfoItem icon={Mail} title="E-mail">
          {contactInfo.email ? (
            <a
              href={`mailto:${contactInfo.email}`}
              className="transition hover:text-white"
            >
              {contactInfo.email}
            </a>
          ) : (
            <p>Entre em contato por e-mail.</p>
          )}
        </ContactInfoItem>

        <ContactInfoItem icon={Phone} title="Telefone / WhatsApp">
          {whatsappNumber ? (
            <a
              href={`https://wa.me/${whatsappNumber}`}
              target="_blank"
              rel="noopener noreferrer"
              className="transition hover:text-white"
            >
              {formattedPhone}
            </a>
          ) : (
            <p>Entre em contato pelo WhatsApp.</p>
          )}
        </ContactInfoItem>

        <ContactInfoItem icon={MapPin} title="Localização">
          <p>{contactInfo.city}</p>
        </ContactInfoItem>

        <ContactInfoItem icon={Clock3} title="Atendimento">
          <p>Entre em contato para consultar nossos horários.</p>
        </ContactInfoItem>
      </div>

      {whatsappNumber && (
        <div className="mt-14 border-t border-white/15 pt-8">
          <p className="text-sm text-white/55">
            Prefere uma conversa mais direta?
          </p>

          <a
            href={`https://wa.me/${whatsappNumber}`}
            target="_blank"
            rel="noopener noreferrer"
            className="group mt-4 inline-flex items-center gap-3 text-sm font-medium"
          >
            <MessageCircle size={18} />
            Fale conosco pelo WhatsApp
            <ArrowUpRight
              size={16}
              className="transition-transform group-hover:translate-x-1 group-hover:-translate-y-1"
            />
          </a>
        </div>
      )}
    </aside>
  );
}
