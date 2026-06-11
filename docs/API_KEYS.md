# API Key Configuration

All external integrations are **optional**. RPlayer works perfectly without any API key.

---

## Last.fm — Scrobbling

Record the songs you listen to on your Last.fm profile.

### Steps

1. Create an account at https://www.last.fm (if you don't have one)
2. Create an API application at https://www.last.fm/api/account/create
   - Application name: `RPlayer` (or whatever name you want)
   - Application description: any text
3. Copy the **API key** and the **Shared secret**
4. Edit `src/lastfm.rs`:

```rust
const API_KEY:    &str = "your_api_key_here";
const API_SECRET: &str = "your_shared_secret_here";
```

5. Recompile: `cargo build --release`

6. In RPlayer: View → Settings → Last.fm section
   - Enter username and password
   - Click "Authenticate"

### What is recorded

- "Now Playing" when loading a file
- Scrobble when playing at least 30 seconds **and** 50% of the duration (or 4 minutes, whichever is less) — Last.fm standard

---

## OpenSubtitles — Automatic subtitle download

Search and download subtitles directly from OpenSubtitles.org.

### Free plan

- 5 downloads per day
- Access to more than 7 million subtitles in 100+ languages

### Steps

1. Create a free account at https://www.opensubtitles.com/en/consumers
2. Go to https://www.opensubtitles.com/en/consumers → "API"
3. Copy the **Consumer API key**
4. Edit `src/opensubtitles.rs`:

```rust
const API_KEY: &str = "your_api_key_here";
```

5. Recompile: `cargo build --release`

### Use in RPlayer

View → "Download subtitles" → write the title → select language → Search → select result → download.

The subtitle is saved in the same directory as the video file and loaded automatically.

---

## Donations (Patreon)

If you want to add a donation banner to your player distribution, edit `src/donation.rs`:

```rust
const PATREON_URL: &str = "https://patreon.com/your_username";
```

The banner appears at the bottom of the window and can be closed permanently.
