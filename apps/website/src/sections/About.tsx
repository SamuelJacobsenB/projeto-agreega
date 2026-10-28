import aboutImage from "../assets/images/about-image.jpeg";

export function About() {
  return (
    <section
      id="sobre"
      className="bg-white px-6 py-24 text-(--color-light-black) sm:px-12 lg:px-24 lg:py-32"
    >
      <div className="mx-auto">
        <div className="grid items-center gap-16 lg:grid-cols-2 lg:gap-20">
          <div className="flex flex-col items-start">
            <span className="mb-6 text-sm font-medium uppercase tracking-[0.2em] text-gray-500">
              Sobre a Agreega
            </span>

            <h2 className="max-w-xl text-4xl font-light leading-tight sm:text-4xl lg:text-5xl">
              Projetar exige mais do que boas ideias.
              <span className="mt-2 block font-medium">
                Exige decisões bem fundamentadas.
              </span>
            </h2>

            <p className="mt-8 max-w-lg text-base leading-8 text-gray-600">
              Em São Gabriel da Palha, a Agreega reúne engenharia e arquitetura
              em uma abordagem que considera cada projeto por inteiro: do
              planejamento inicial às soluções técnicas e espaciais.
            </p>

            <p className="mt-4 max-w-lg text-base leading-8 text-gray-600">
              Nosso trabalho parte das características de cada espaço, das
              necessidades do cliente e dos aspectos técnicos envolvidos,
              buscando soluções coerentes tanto no projeto quanto na execução.
            </p>
          </div>

          <div className="relative aspect-4/5 overflow-hidden bg-gray-100">
            <img
              src={aboutImage}
              alt="Projeto arquitetônico contemporâneo"
              className="h-full w-full object-cover transition-transform duration-700 hover:scale-105"
              loading="lazy"
            />
          </div>
        </div>
      </div>
    </section>
  );
}
