import { useEffect, useState } from "react";

import { ArrowRight, Menu, X } from "lucide-react";

import { routes } from "../../constants";

export function Header() {
  const [activeSection, setActiveSection] = useState("#home");
  const [isScrolled, setIsScrolled] = useState(false);
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setIsScrolled(window.scrollY > 50);
    };

    window.addEventListener("scroll", handleScroll, { passive: true });
    handleScroll();

    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  useEffect(() => {
    const sections = routes
      .filter(({ href }) => href.startsWith("#"))
      .map(({ href }) => document.querySelector(href))
      .filter((section): section is Element => section !== null);

    const observer = new IntersectionObserver(
      (entries) => {
        const visibleSection = entries.find((entry) => entry.isIntersecting);

        if (visibleSection) {
          setActiveSection(`#${visibleSection.target.id}`);
        }
      },
      { rootMargin: "-20% 0px -60% 0px" },
    );

    sections.forEach((section) => observer.observe(section));

    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    document.body.style.overflow = isMenuOpen ? "hidden" : "";

    return () => {
      document.body.style.overflow = "";
    };
  }, [isMenuOpen]);

  const closeMenu = () => setIsMenuOpen(false);

  return (
    <header
      className={`fixed inset-x-0 top-0 z-50 transition-colors duration-300 ${
        isScrolled || isMenuOpen
          ? "bg-(--color-light-black) shadow-lg"
          : "bg-transparent"
      }`}
    >
      <div className="flex items-center justify-between px-6 py-4 lg:px-24">
        <a href="#home" onClick={closeMenu} className="shrink-0">
          <img
            src="/images/logo.svg"
            alt="Agreega Engenharia e Arquitetura"
            className="w-48 lg:w-56"
          />
        </a>

        <div className="hidden items-center gap-12 lg:flex">
          <nav>
            <ul className="flex items-center gap-8 xl:gap-10">
              {routes.map(({ label, href }) => {
                const isActive = activeSection === href;

                return (
                  <li key={href}>
                    <a
                      href={href}
                      aria-current={isActive ? "location" : undefined}
                      className={`relative block py-2 text-lg transition-colors duration-300 after:absolute after:inset-x-0 after:bottom-0 after:h-0.5 after:bg-(--color-light-yellow) after:transition-transform after:duration-300 ${
                        isActive
                          ? "text-white after:scale-x-100"
                          : "text-gray-200 after:scale-x-0 hover:text-white hover:after:scale-x-100"
                      }`}
                    >
                      {label}
                    </a>
                  </li>
                );
              })}
            </ul>
          </nav>

          <a
            href="https://wa.me/5527998250195"
            target="_blank"
            rel="noopener noreferrer"
            className="group inline-flex shrink-0 items-center gap-2 rounded-full border border-white px-6 py-2 text-lg text-white transition-all duration-300 hover:-translate-y-1 hover:bg-white hover:text-(--color-light-black)"
          >
            Fale conosco
            <ArrowRight
              size={18}
              className="transition-transform duration-300 group-hover:translate-x-1"
            />
          </a>
        </div>

        <button
          type="button"
          aria-label={isMenuOpen ? "Fechar menu" : "Abrir menu"}
          aria-expanded={isMenuOpen}
          aria-controls="mobile-menu"
          onClick={() => setIsMenuOpen((open) => !open)}
          className="rounded-lg p-2 text-white transition-colors hover:bg-white/10 lg:hidden"
        >
          {isMenuOpen ? <X size={28} /> : <Menu size={28} />}
        </button>
      </div>

      <nav
        id="mobile-menu"
        aria-hidden={!isMenuOpen}
        className={`grid overflow-hidden transition-all duration-300 ease-in-out lg:hidden ${
          isMenuOpen
            ? "grid-rows-[1fr] opacity-100"
            : "pointer-events-none grid-rows-[0fr] opacity-0"
        }`}
      >
        <div className="min-h-0">
          <ul className="flex flex-col gap-2 px-6 pb-6 pt-2">
            {routes.map(({ label, href }) => {
              const isActive = activeSection === href;

              return (
                <li key={href}>
                  <a
                    href={href}
                    tabIndex={isMenuOpen ? 0 : -1}
                    aria-current={isActive ? "location" : undefined}
                    onClick={closeMenu}
                    className={`block rounded-lg px-4 py-3 text-base transition-colors ${
                      isActive
                        ? "bg-white/10 text-(--color-light-yellow)"
                        : "text-gray-100 hover:bg-white/10 hover:text-white"
                    }`}
                  >
                    {label}
                  </a>
                </li>
              );
            })}

            <li className="pt-2">
              <a
                href="https://wa.me/5527998250195"
                target="_blank"
                rel="noopener noreferrer"
                onClick={closeMenu}
                className="group flex items-center justify-center gap-2 rounded-full border border-white px-6 py-3 text-white transition-colors hover:bg-white hover:text-(--color-light-black)"
              >
                Fale conosco
                <ArrowRight
                  size={18}
                  className="transition-transform duration-300 group-hover:translate-x-1"
                />
              </a>
            </li>
          </ul>
        </div>
      </nav>
    </header>
  );
}
