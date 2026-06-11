# Web de RPlayer

Landing estática lista para publicar.

## Archivos

- `index.html`
- `styles.css`
- `app.js`
- `ads.js`
- `privacy.html`
- `terms.html`
- `assets/icon-rp.png`

## Personalizar antes de publicar

Reemplaza estos placeholders en `index.html`:

- `https://github.com/TU_ORG/rplayer`
- `https://github.com/TU_ORG/rplayer/releases/latest`
- `https://patreon.com/TU_USUARIO`
- `contacto@rplayer.app`

## Publicidad (AdSense u otra red)

El sitio ya incluye zonas de anuncios en todas las páginas (`index`, `privacy`, `terms`).

Para activarlas:

1. Abre `ads.js`.
2. Reemplaza `ADSENSE_CLIENT` por tu ID real (`ca-pub-...`).
3. Reemplaza los `data-ad-slot` de cada bloque por slots reales.

Ejemplo AdSense (reemplaza `ca-pub-XXXXXXXXXX`):

```html
<script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-XXXXXXXXXX" crossorigin="anonymous"></script>
<ins class="adsbygoogle" style="display:block" data-ad-client="ca-pub-XXXXXXXXXX" data-ad-slot="1234567890" data-ad-format="auto" data-full-width-responsive="true"></ins>
<script>(adsbygoogle = window.adsbygoogle || []).push({});</script>
```

## Despliegue rápido

### GitHub Pages

1. Crear repo (o rama) para sitio web.
2. Subir contenido de carpeta `website/`.
3. Activar GitHub Pages (branch `main`, folder `/root` o `/docs`).

### Netlify / Vercel

1. Importar repo.
2. Configurar directorio raíz: `website`.
3. Deploy.

## Recomendación legal

Si usas anuncios, agrega:

- Política de privacidad
- Aviso de cookies
- Aviso de terceros/publicidad personalizada
