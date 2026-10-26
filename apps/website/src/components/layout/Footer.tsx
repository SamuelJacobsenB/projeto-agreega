import { ArrowRight, ArrowUpRight } from "lucide-react";

export function Footer() {
  return (
    <footer className="w-full bg-(--color-light-black) text-white">
      <div className="mx-auto px-6 py-10 sm:px-10 lg:px-24">
        <div className="flex flex-col justify-between gap-8 pb-4 sm:flex-row sm:items-center">
          <a href="#home" aria-label="Agreega — Página inicial">
            <img
              src="/images/logo.svg"
              alt="Agreega Engenharia e Arquitetura"
              className="w-40"
            />
          </a>

          <a
            href="https://wa.me/5527998250195"
            target="_blank"
            rel="noopener noreferrer"
            className="group inline-flex w-fit items-center gap-2 text-sm transition-colors duration-300 hover:text-(--color-light-yellow)"
          >
            Fale conosco
            <ArrowUpRight
              size={16}
              className="transition-transform duration-300 group-hover:-translate-y-0.5 group-hover:translate-x-0.5"
            />
          </a>
        </div>

        <div className="mt-8 flex flex-col justify-between gap-4 border-t border-white/10 pt-5 sm:flex-row sm:items-center">
          <p className="text-xs text-gray-500">
            © 2026 Agreega. Todos os direitos reservados.
          </p>

          <a
            href="#home"
            className="group inline-flex w-fit items-center gap-2 text-sm text-gray-400 transition-colors duration-300 hover:text-white"
          >
            Voltar ao topo
            <ArrowRight
              size={16}
              className="-rotate-90 transition-transform duration-300 group-hover:-translate-y-0.5"
            />
          </a>
        </div>
      </div>
    </footer>
  );
}
