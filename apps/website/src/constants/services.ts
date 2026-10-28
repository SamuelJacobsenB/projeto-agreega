import { Building2, HardHat, Ruler } from "lucide-react";

import arquitetura from "../assets/images/arquitetura.jpeg";
import engenharia from "../assets/images/engenharia.jpg";
import planejamento from "../assets/images/planejamento.jpeg";

export const services = [
  {
    number: "01",
    title: "Projetos Arquitetônicos",
    description:
      "Estudo e desenvolvimento de espaços, considerando distribuição, dimensões, estética e necessidades de uso.",
    image: arquitetura,
    imageAlt: "Planta arquitetônica impressa com desenhos técnicos",
    icon: Ruler,
    category: "ARQUITETURA",
    layout: "image-left",
  },
  {
    number: "02",
    title: "Projetos de Engenharia",
    description:
      "Desenvolvimento de soluções técnicas e desenhos que orientam as diferentes etapas de um empreendimento.",
    image: engenharia,
    imageAlt: "Desenho técnico de engenharia em software CAD",
    icon: Building2,
    category: "ENGENHARIA",
    layout: "image-right",
  },
  {
    number: "03",
    title: "Planejamento e Acompanhamento",
    description:
      "Organização das etapas do projeto e suporte técnico conforme as necessidades de cada obra.",
    image: planejamento,
    imageAlt: "Prancha técnica com cortes, cotas e detalhes construtivos",
    icon: HardHat,
    category: "PLANEJAMENTO",
    layout: "image-left",
  },
];
