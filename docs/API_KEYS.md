# Configuración de API Keys

Todas las integraciones externas son **opcionales**. RPlayer funciona perfectamente sin ninguna API key.

---

## Last.fm — Scrobbling

Registra las canciones que escuchas en tu perfil de Last.fm.

### Pasos

1. Crear cuenta en https://www.last.fm (si no tienes una)
2. Crear una API application en https://www.last.fm/api/account/create
   - Application name: `RPlayer` (o el nombre que quieras)
   - Application description: cualquier texto
3. Copiar la **API key** y el **Shared secret**
4. Editar `src/lastfm.rs`:

```rust
const API_KEY:    &str = "tu_api_key_aquí";
const API_SECRET: &str = "tu_shared_secret_aquí";
```

5. Recompilar: `cargo build --release`

6. En RPlayer: Vista → Configuración → sección Last.fm
   - Ingresar usuario y contraseña
   - Hacer clic en "Autenticar"

### Qué se registra

- "Now Playing" al cargar un archivo
- Scrobble cuando se reproducen al menos 30 segundos **y** el 50% de la duración (o 4 minutos, lo que sea menor) — estándar de Last.fm

---

## OpenSubtitles — Descarga automática de subtítulos

Busca y descarga subtítulos directamente desde OpenSubtitles.org.

### Plan gratuito

- 5 descargas por día
- Acceso a más de 7 millones de subtítulos en 100+ idiomas

### Pasos

1. Crear cuenta gratuita en https://www.opensubtitles.com/en/consumers
2. Ir a https://www.opensubtitles.com/en/consumers → "API"
3. Copiar la **Consumer API key**
4. Editar `src/opensubtitles.rs`:

```rust
const API_KEY: &str = "tu_api_key_aquí";
```

5. Recompilar: `cargo build --release`

### Uso en RPlayer

Vista → "Bajar subtítulos" → escribir el título → seleccionar idioma → Buscar → seleccionar resultado → descargar.

El subtítulo se guarda en el mismo directorio del archivo de video y se carga automáticamente.

---

## Donaciones (Patreon)

Si quieres añadir un banner de donación a tu distribución del reproductor, editar `src/donation.rs`:

```rust
const PATREON_URL: &str = "https://patreon.com/tu_usuario";
```

El banner aparece en la parte inferior de la ventana y puede cerrarse permanentemente.
