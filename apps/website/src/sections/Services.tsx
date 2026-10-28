import { ArrowUpRight } from "lucide-react";
import { services } from "../constants";

export function Services() {
  return (
    <section
      id="servicos"
      className="overflow-hidden bg-(--color-light-black) px-6 py-24 text-white sm:px-12 lg:px-24 lg:py-32"
    >
      <div className="mx-auto max-w-7xl">
        <div className="mb-16 flex flex-col gap-8 lg:mb-20 lg:flex-row lg:items-end lg:justify-between">
          <div className="w-full lg:w-3/5">
            <span className="mb-5 flex items-center gap-3 text-xs font-medium uppercase tracking-[0.25em] text-gray-400">
              <span className="h-px w-8 bg-(--color-light-yellow)" />
              Nossas soluções
            </span>

            <h2 className="max-w-3xl text-4xl font-light leading-tight sm:text-5xl lg:text-7xl">
              Do desenho à
              <span className="block font-medium text-(--color-light-yellow)">
                solução técnica.
              </span>
            </h2>
          </div>

          <p className="w-full max-w-lg text-base leading-8 text-gray-300 lg:w-2/5 lg:pb-2">
            Cada etapa exige decisões diferentes. Reunimos arquitetura,
            engenharia e planejamento para atender às particularidades de cada
            projeto.
          </p>
        </div>

        <div className="flex flex-col gap-20 lg:gap-28">
          {services.map((service) => {
            const Icon = service.icon;
            const imageOnRight = service.layout === "image-right";

            return (
              <article
                key={service.number}
                className="flex flex-col items-center gap-8 lg:flex-row lg:gap-16"
              >
                <div
                  className={`group relative w-full lg:w-1/2 lg:shrink-0 ${
                    imageOnRight ? "lg:order-2" : ""
                  }`}
                >
                  <div className="absolute -bottom-3 -right-3 h-24 w-24 border-b border-r border-(--color-light-yellow) sm:-bottom-4 sm:-right-4" />

                  <div className="relative aspect-5/4 overflow-hidden bg-white/5">
                    <img
                      src={service.image}
                      alt={service.imageAlt}
                      loading="lazy"
                      className="h-full w-full object-cover transition-transform duration-700 group-hover:scale-[1.03]"
                    />

                    <div className="absolute inset-0 bg-black/10 transition-colors duration-500 group-hover:bg-transparent" />

                    <span className="absolute left-5 top-5 bg-(--color-light-black)/90 px-3 py-2 text-[10px] font-medium tracking-[0.2em] text-white backdrop-blur-sm">
                      {service.category}
                    </span>

                    <span className="absolute bottom-5 right-5 flex h-11 w-11 items-center justify-center bg-(--color-light-yellow) text-(--color-light-black)">
                      <Icon size={22} strokeWidth={1.5} />
                    </span>
                  </div>
                </div>

                <div
                  className={`flex w-full flex-col items-start lg:w-1/2 ${
                    imageOnRight ? "lg:order-1 lg:pl-8" : "lg:pr-8"
                  }`}
                >
                  <span className="text-sm font-medium tracking-widest text-gray-500">
                    {service.number}
                  </span>

                  <h3 className="mt-5 max-w-xl text-3xl font-light leading-tight sm:text-4xl lg:text-5xl">
                    {service.title}
                  </h3>

                  <p className="mt-6 max-w-lg text-base leading-8 text-gray-300">
                    {service.description}
                  </p>

                  <a
                    href="#contato"
                    className="group mt-8 inline-flex items-center gap-3 border-b border-white/20 pb-3 text-sm font-medium transition-colors duration-300 hover:border-(--color-light-yellow) hover:text-(--color-light-yellow)"
                  >
                    Converse com a Agreega
                    <ArrowUpRight
                      size={17}
                      className="transition-transform duration-300 group-hover:-translate-y-0.5 group-hover:translate-x-0.5"
                    />
                  </a>
                </div>
              </article>
            );
          })}
        </div>

        <div className="relative mt-24 flex flex-col gap-7 overflow-hidden bg-(--color-light-yellow) px-6 py-10 text-(--color-light-black) sm:px-10 sm:py-12 lg:mt-32 lg:flex-row lg:items-center lg:justify-between lg:px-14">
          <div>
            <span className="text-xs font-medium uppercase tracking-[0.2em] opacity-70">
              Seu projeto começa aqui
            </span>

            <h3 className="mt-4 max-w-2xl text-2xl font-light leading-snug sm:text-3xl">
              Tem um projeto em mente?
              <span className="block font-medium">
                Vamos conversar sobre ele.
              </span>
            </h3>
          </div>

          <a
            href="#contato"
            className="group inline-flex w-fit shrink-0 items-center gap-3 border border-(--color-light-black)/20 px-6 py-4 text-sm font-medium transition-colors duration-300 hover:bg-(--color-light-black) hover:text-white lg:ml-10"
          >
            Entre em contato
            <ArrowUpRight
              size={18}
              className="transition-transform duration-300 group-hover:-translate-y-0.5 group-hover:translate-x-0.5"
            />
          </a>
        </div>
      </div>
    </section>
  );
}
