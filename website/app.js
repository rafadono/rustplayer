const i18n = {
  es: {
    navDownloads: "Descargas",
    navFeatures: "Características",
    navChangelog: "Changelog",
    navSupport: "Soporte",
    navContact: "Contacto",
    heroTag: "Hecho en Rust · Linux + Windows · Sin telemetría invasiva",
    heroTitle: "El reproductor moderno, libre y potente para audio y video",
    heroLead:
      "RPlayer nace para ofrecer control total de reproducción, alto rendimiento y una experiencia limpia para usuarios avanzados y usuarios cotidianos.",
    heroCtaDownload: "Descargar ahora",
    heroCtaAbout: "Conocer el proyecto",
    adsTopTitle: "Publicidad",
    adsTopSlot: "Espacio para anuncio (728x90)",
    adsTopHint: "Inserta aquí Google AdSense o red publicitaria compatible.",
    goalTitle: "Objetivo del proyecto",
    goalText:
      "Construir un reproductor confiable, configurable y de código abierto, capaz de manejar reproducción local, streams, podcast, karaoke, subtítulos y herramientas avanzadas de audio/video, manteniendo seguridad y estabilidad como prioridades.",
    downloadsTitle: "Descargas",
    dlWinText: "Build oficial para Windows x64.",
    dlWinBtn: "Descargar .exe",
    dlLinuxText: "Binarios y guía de instalación por distro.",
    dlLinuxBtn: "Descargar Linux",
    dlSrcTitle: "Código fuente",
    dlSrcText: "Compila desde fuente y contribuye al proyecto.",
    dlSrcBtn: "Ver en GitHub",
    featuresTitle: "Características destacadas",
    basicTitle: "Lo básico (que cualquier reproductor debe tener)",
    basic1: "Play/pausa, volumen, mute, seek y atajos de teclado.",
    basic2: "Soporte para formatos populares de video y audio.",
    basic3: "Playlist, control de velocidad y pantalla completa.",
    basic4: "Subtítulos y cambio de pistas de audio.",
    proTitle: "Lo que diferencia a RPlayer",
    pro1: "Ecualizador paramétrico (PEQ), presets, preamp y anti-clipping.",
    pro2: "Controles avanzados de imagen: deinterlace, integer scaling y más.",
    pro3: "Diagnóstico de codecs en runtime y herramientas de troubleshooting.",
    pro4: "Radio/podcast/streams, historial, marcadores, notas y Up Next.",
    pro5: "Actualizaciones auto/manual con rollback si algo falla.",
    pro6: "Proyecto open source hecho en Rust: rendimiento y estabilidad.",
    supportTitle: "Apoya el proyecto",
    supportText:
      "RPlayer se mantiene gracias al tiempo de desarrollo y al apoyo de la comunidad. Si te sirve el proyecto, puedes apoyar su evolución continua.",
    supportBtn: "Apoyar en Patreon",
    changelogTitle: "Changelog y progreso",
    changelogLead:
      "Resumen del avance del proyecto. Incluye funciones implementadas y hitos reconstruidos cuando no hay fecha exacta registrada.",
    implementedTitle: "Implementado (historial confirmado)",
    impl1: "Integración sólida con libmpv en Windows/Linux y fallback robusto para eventos.",
    impl2: "UI bilingüe (Español/English) y base de traducciones por keys.",
    impl3: "Reporte de bugs desde la app con datos técnicos y canal configurable.",
    impl4: "Actualizaciones auto/manual con rollback automático si la nueva versión falla.",
    impl5: "Ecualizador paramétrico (PEQ): 6 filtros, presets, preamp y anti-clipping.",
    impl6: "Diagnóstico de codecs en runtime y panel dedicado.",
    impl7: "Controles avanzados: integer scaling, subtítulos mejorados y ajustes visuales.",
    impl8: "Soporte ampliado de streaming: radio, podcasts y utilidades asociadas.",
    reconstructedTitle: "Hitos reconstruidos (sin fecha exacta)",
    rec1: "Fase inicial: reproducción base + atajos + playlist (MVP usable).",
    rec2: "Fase media: subtítulos, historial, marcadores, paneles laterales y personalización.",
    rec3: "Fase avanzada: herramientas pro de audio/video y mayor estabilidad multiplataforma.",
    rec4: "Fase producto: updates in-app, bug reporting, web oficial y estrategia de monetización.",
    rec5: "Próxima lógica natural: más telemetría opt-in, métricas de uso y roadmap público por versión.",
    changelogNote:
      "¿Quieres acelerar estos hitos? Tu apoyo en Patreon impacta directamente el ritmo de desarrollo.",
    changelogPatreonBtn: "Apoyar en Patreon",
    adsMidTitle: "Publicidad",
    adsMidSlot: "Espacio para anuncio (300x250)",
    adsMidHint: "Recomendado: 1 bloque en mitad de contenido y 1 en footer.",
    contactTitle: "Contacto y bugs",
    contactText1: "Para soporte, reportes de error o propuestas de colaboración:",
    contactText2: "También puedes abrir issues en",
    monetTitle: "Monetización y transparencia",
    monetText:
      "Esta web puede mostrar publicidad para financiar mantenimiento y nuevas funciones. Se recomienda incluir política de privacidad/cookies y aviso de terceros (AdSense u otra red).",
    monetBtn: "Ver zonas de anuncios",
    footerText: (y) => `© ${y} RPlayer. Hecho con Rust, egui y libmpv.`,
    langButton: "EN",
    pageTitle: "RPlayer | Reproductor libre en Rust",
  },
  en: {
    navDownloads: "Downloads",
    navFeatures: "Features",
    navChangelog: "Changelog",
    navSupport: "Support",
    navContact: "Contact",
    heroTag: "Built with Rust · Linux + Windows · No invasive telemetry",
    heroTitle: "A modern, free and powerful audio/video player",
    heroLead:
      "RPlayer is designed to deliver full playback control, strong performance, and a clean experience for both power users and everyday users.",
    heroCtaDownload: "Download now",
    heroCtaAbout: "Learn about the project",
    adsTopTitle: "Advertising",
    adsTopSlot: "Ad slot (728x90)",
    adsTopHint: "Place Google AdSense or any compatible ad network here.",
    goalTitle: "Project goal",
    goalText:
      "Build a reliable, configurable, open-source player that handles local playback, streams, podcasts, karaoke, subtitles, and advanced audio/video tools while keeping security and stability as top priorities.",
    downloadsTitle: "Downloads",
    dlWinText: "Official Windows x64 build.",
    dlWinBtn: "Download .exe",
    dlLinuxText: "Binaries and installation guides by distro.",
    dlLinuxBtn: "Download Linux",
    dlSrcTitle: "Source code",
    dlSrcText: "Build from source and contribute to the project.",
    dlSrcBtn: "View on GitHub",
    featuresTitle: "Key features",
    basicTitle: "The basics (every good player should have)",
    basic1: "Play/pause, volume, mute, seek, and keyboard shortcuts.",
    basic2: "Support for common video and audio formats.",
    basic3: "Playlist, speed control, and fullscreen mode.",
    basic4: "Subtitles and audio track selection.",
    proTitle: "What makes RPlayer different",
    pro1: "Parametric EQ (PEQ), presets, preamp, and anti-clipping.",
    pro2: "Advanced image controls: deinterlace, integer scaling, and more.",
    pro3: "Runtime codec diagnostics and troubleshooting tools.",
    pro4: "Radio/podcast/streams, history, bookmarks, notes, and Up Next.",
    pro5: "Auto/manual updates with rollback if something fails.",
    pro6: "Open-source and built in Rust for performance and stability.",
    supportTitle: "Support the project",
    supportText:
      "RPlayer is sustained by development time and community support. If the project is useful to you, you can help us keep improving it.",
    supportBtn: "Support on Patreon",
    changelogTitle: "Changelog and progress",
    changelogLead:
      "Project progress summary. Includes implemented features and backfilled milestones when exact dates are unavailable.",
    implementedTitle: "Implemented (confirmed history)",
    impl1: "Solid libmpv integration on Windows/Linux with robust event fallback.",
    impl2: "Bilingual UI (Spanish/English) with key-based translation foundation.",
    impl3: "In-app bug reporting with technical context and configurable report channel.",
    impl4: "Auto/manual updates with automatic rollback when a new version fails.",
    impl5: "Parametric EQ (PEQ): 6 filters, presets, preamp, and anti-clipping.",
    impl6: "Runtime codec diagnostics and dedicated panel.",
    impl7: "Advanced controls: integer scaling, improved subtitles, and visual tuning.",
    impl8: "Expanded streaming support: radio, podcasts, and related utilities.",
    reconstructedTitle: "Backfilled milestones (no exact date recorded)",
    rec1: "Initial phase: base playback + shortcuts + playlist (usable MVP).",
    rec2: "Middle phase: subtitles, history, bookmarks, side panels, and customization.",
    rec3: "Advanced phase: pro audio/video tools and stronger cross-platform stability.",
    rec4: "Product phase: in-app updates, bug reporting, official website, and monetization strategy.",
    rec5: "Natural next step: more opt-in telemetry, usage metrics, and public version roadmap.",
    changelogNote:
      "Want to speed up these milestones? Patreon support directly improves development pace.",
    changelogPatreonBtn: "Support on Patreon",
    adsMidTitle: "Advertising",
    adsMidSlot: "Ad slot (300x250)",
    adsMidHint: "Recommended: 1 block in content middle and 1 in footer.",
    contactTitle: "Contact and bug reports",
    contactText1: "For support, bug reports, or collaboration proposals:",
    contactText2: "You can also open issues on",
    monetTitle: "Monetization and transparency",
    monetText:
      "This website may show ads to fund maintenance and new features. It is recommended to include a privacy/cookie policy and third-party advertising notice.",
    monetBtn: "See ad zones",
    footerText: (y) => `© ${y} RPlayer. Built with Rust, egui and libmpv.`,
    langButton: "ES",
    pageTitle: "RPlayer | Free media player in Rust",
  },
};

const setText = (id, value) => {
  const el = document.getElementById(id);
  if (!el) return;
  el.textContent = value;
};

function applyLanguage(lang) {
  const t = i18n[lang] || i18n.es;
  const year = new Date().getFullYear();

  document.documentElement.lang = lang;
  document.title = t.pageTitle;

  setText("nav-downloads", t.navDownloads);
  setText("nav-features", t.navFeatures);
  setText("nav-changelog", t.navChangelog);
  setText("nav-support", t.navSupport);
  setText("nav-contact", t.navContact);
  setText("hero-tag", t.heroTag);
  setText("hero-title", t.heroTitle);
  setText("hero-lead", t.heroLead);
  setText("hero-cta-download", t.heroCtaDownload);
  setText("hero-cta-about", t.heroCtaAbout);
  setText("ads-top-title", t.adsTopTitle);
  setText("ads-top-slot", t.adsTopSlot);
  setText("ads-top-hint", t.adsTopHint);
  setText("goal-title", t.goalTitle);
  setText("goal-text", t.goalText);
  setText("downloads-title", t.downloadsTitle);
  setText("dl-win-text", t.dlWinText);
  setText("dl-win-btn", t.dlWinBtn);
  setText("dl-linux-text", t.dlLinuxText);
  setText("dl-linux-btn", t.dlLinuxBtn);
  setText("dl-src-title", t.dlSrcTitle);
  setText("dl-src-text", t.dlSrcText);
  setText("dl-src-btn", t.dlSrcBtn);
  setText("features-title", t.featuresTitle);
  setText("basic-title", t.basicTitle);
  setText("basic-1", t.basic1);
  setText("basic-2", t.basic2);
  setText("basic-3", t.basic3);
  setText("basic-4", t.basic4);
  setText("pro-title", t.proTitle);
  setText("pro-1", t.pro1);
  setText("pro-2", t.pro2);
  setText("pro-3", t.pro3);
  setText("pro-4", t.pro4);
  setText("pro-5", t.pro5);
  setText("pro-6", t.pro6);
  setText("support-title", t.supportTitle);
  setText("support-text", t.supportText);
  setText("support-btn", t.supportBtn);
  setText("changelog-title", t.changelogTitle);
  setText("changelog-lead", t.changelogLead);
  setText("implemented-title", t.implementedTitle);
  setText("impl-1", t.impl1);
  setText("impl-2", t.impl2);
  setText("impl-3", t.impl3);
  setText("impl-4", t.impl4);
  setText("impl-5", t.impl5);
  setText("impl-6", t.impl6);
  setText("impl-7", t.impl7);
  setText("impl-8", t.impl8);
  setText("reconstructed-title", t.reconstructedTitle);
  setText("rec-1", t.rec1);
  setText("rec-2", t.rec2);
  setText("rec-3", t.rec3);
  setText("rec-4", t.rec4);
  setText("rec-5", t.rec5);
  setText("changelog-note", t.changelogNote);
  setText("changelog-patreon-btn", t.changelogPatreonBtn);
  setText("ads-mid-title", t.adsMidTitle);
  setText("ads-mid-slot", t.adsMidSlot);
  setText("ads-mid-hint", t.adsMidHint);
  setText("contact-title", t.contactTitle);
  setText("contact-text-1", t.contactText1);
  setText("contact-text-2", t.contactText2);
  setText("monet-title", t.monetTitle);
  setText("monet-text", t.monetText);
  setText("monet-btn", t.monetBtn);
  setText("footer-text", t.footerText(year));
  setText("lang-toggle", t.langButton);

  localStorage.setItem("rplayer-site-lang", lang);
}

const saved = localStorage.getItem("rplayer-site-lang");
applyLanguage(saved === "en" ? "en" : "es");

document.getElementById("lang-toggle")?.addEventListener("click", () => {
  const current = document.documentElement.lang === "en" ? "en" : "es";
  applyLanguage(current === "en" ? "es" : "en");
});
