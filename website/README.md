# RPlayer website

Static landing ready to publish.

## Files

- `index.html`
- `styles.css`
- `app.js`
- `ads.js`
- `privacy.html`
- `terms.html`
- `assets/icon-rp.png`

## Customize before publishing

Replace these placeholders in `index.html`:

- `https://github.com/TU_ORG/rplayer`
- `https://github.com/TU_ORG/rplayer/releases/latest`
- `https://patreon.com/TU_USUARIO`
- `contacto@rplayer.app`

## Advertising (AdSense or other network)

The site already includes ad areas on all pages (`index`, `privacy`, `terms`).

To activate them:

1. Open `ads.js`.
2. Replace `ADSENSE_CLIENT` with your real ID (`ca-pub-...`).
3. Replaces the `data-ad-slot` of each block with real slots.

AdSense example (replaces `ca-pub-XXXXXXXXXX`):

```html
<script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-XXXXXXXXXX" crossorigin="anonymous"></script>
<ins class="adsbygoogle" style="display:block" data-ad-client="ca-pub-XXXXXXXXXX" data-ad-slot="1234567890" data-ad-format="auto" data-full-width-responsive="true"></ins>
<script>(adsbygoogle = window.adsbygoogle || []).push({});</script>
```

## Rapid deployment

### GitHub Pages

1. Create repo (or branch) for website.
2. Upload contents of folder `website/`.
3. Activate GitHub Pages (branch `main`, folder `/root` or `/docs`).

### Netlify / Vercel

1. Import repo.
2. Set root directory: `website`.
3. Deploy.

## Legal recommendation

If you use ads, add:

- Privacy Policy
- Cookie notice
- Third Party Notice/Personalized Advertising
