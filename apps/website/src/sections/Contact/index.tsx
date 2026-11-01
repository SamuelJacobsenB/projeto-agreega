import { useState } from "react";

import type { ClientType } from "../../types";

import { ContactForm, ContactInfo } from "./components";

export function Contact() {
  const [clientType, setClientType] = useState<ClientType>("individual");

  return (
    <section
      id="contato"
      className="border-b border-(--color-gray) bg-(--color-light-black) px-6 py-24 text-white sm:px-12 lg:px-24 lg:py-32"
    >
      <div className="mx-auto max-w-7xl">
        <header className="mb-16 max-w-2xl">
          <span className="text-xs font-medium tracking-[0.25em] text-white/50 uppercase">
            Contato
          </span>

          <h2 className="mt-5 text-4xl leading-tight font-light tracking-tight sm:text-5xl lg:text-6xl">
            Vamos conversar sobre seu{" "}
            <span className="text-white/60">próximo projeto?</span>
          </h2>

          <p className="mt-6 max-w-lg text-sm leading-relaxed text-white/60 sm:text-base">
            Estamos prontos para ouvir suas ideias e ajudar a transformá-las em
            realidade. Entre em contato e vamos encontrar a melhor solução para
            você.
          </p>
        </header>

        <div className="grid gap-16 lg:grid-cols-[1.3fr_0.7fr] lg:gap-24">
          <ContactForm
            clientType={clientType}
            onClientTypeChange={setClientType}
          />

          <ContactInfo />
        </div>
      </div>
    </section>
  );
}
