import { ArrowRight } from "lucide-react";

export function Hero() {
  return (
    <section
      id="home"
      className="relative flex h-dvh w-full flex-col justify-center gap-8 bg-[url('/images/hero-background.png')] bg-cover bg-center bg-no-repeat px-6 sm:px-12 lg:px-32 before:absolute before:inset-0 before:bg-black/45"
    >
      <div className="relative z-10 flex flex-col gap-8">
        <span className="flex flex-wrap items-center gap-3 text-xs font-medium uppercase tracking-[0.2em] text-(--color-gray) sm:text-sm">
          <span>Planejamento</span>
          <span className="text-(--color-light-yellow)">/</span>
          <span>Construção</span>
          <span className="text-(--color-light-yellow)">/</span>
          <span>Arquitetura</span>
        </span>

        <h1 className="flex flex-col text-4xl font-light text-white sm:text-6xl lg:text-7xl">
          <span className="text-6xl font-medium tracking-tight text-(--color-light-yellow) sm:text-8xl lg:text-9xl">
            AGREEGA
          </span>
          Engenharia e Arquitetura
        </h1>

        <p className="max-w-xl text-base leading-relaxed text-gray-300 sm:text-lg">
          Soluções em engenharia e arquitetura que aliam precisão técnica,
          funcionalidade e identidade em cada projeto.
        </p>

        <a
          href="#projetos"
          className="flex w-fit items-center gap-2 text-sm font-medium text-(--color-light-black) bg-(--color-light-yellow) transition-all rounded-xs px-6 py-4 duration-300 hover:translate-x-0.5"
        >
          Conheça nossos projetos
          <ArrowRight size={16} />
        </a>
      </div>
    </section>
  );
}
