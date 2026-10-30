import { Footer, Header } from "../components/layout";
import { About, Contact, Hero, Projects, Services } from "../sections";

export function App() {
  return (
    <>
      <Header />

      <main>
        <Hero />
        <About />
        <Services />
        <Projects />
        <Contact />
      </main>

      <Footer />
    </>
  );
}
