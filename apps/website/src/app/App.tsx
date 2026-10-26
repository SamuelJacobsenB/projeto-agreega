import { Footer, Header } from "../components/layout";
import { About, Budget, Contact, Hero, Projects, Services } from "../sections";

export function App() {
  return (
    <>
      <Header />

      <main>
        <Hero />
        <About />
        <Services />
        <Projects />
        <Budget />
        <Contact />
      </main>

      <Footer />
    </>
  );
}
