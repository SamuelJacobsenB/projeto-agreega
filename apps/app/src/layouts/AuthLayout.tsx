import { Outlet } from "react-router-dom";

const year = new Date().getFullYear();

export function AuthLayout() {
  return (
    <main className="flex min-h-screen bg-background">
      <section className="relative hidden flex-1 items-center justify-center overflow-hidden bg-[#252b30] bg-[url('/images/login-background.png')] bg-cover bg-center bg-no-repeat lg:flex">
        <div className="absolute inset-0 bg-black/45" />

        <img
          src="/images/logo.svg"
          alt="Agreega"
          className="relative z-10 h-20 w-auto"
        />
      </section>

      <section className="flex min-h-screen w-full flex-col lg:w-120 lg:shrink-0">
        <header className="p-8 lg:hidden">
          <div
            role="img"
            aria-label="Agreega"
            className="h-10 w-auto bg-foreground"
            style={{
              maskImage: "url('/images/logo.svg')",
              WebkitMaskImage: "url('/images/logo.svg')",
              maskRepeat: "no-repeat",
              WebkitMaskRepeat: "no-repeat",
              maskPosition: "left center",
              WebkitMaskPosition: "left center",
              maskSize: "contain",
              WebkitMaskSize: "contain",
            }}
          />
        </header>

        <div className="flex flex-1 items-center justify-center px-6 py-12">
          <div className="w-full lg:max-w-96">
            <Outlet />
          </div>
        </div>

        <footer className="p-6 text-center text-xs text-muted-foreground">
          © {year} Agreega
        </footer>
      </section>
    </main>
  );
}
