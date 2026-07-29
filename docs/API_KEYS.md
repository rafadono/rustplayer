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
4. Set the environment variables before building, so the keys get baked into
   the binary (obfuscated with `obfstr`, see [src/services/lastfm.rs](../src/services/lastfm.rs)):

```bash
# Linux / macOS
export RUSTPLAYER_LASTFM_KEY="your_api_key_here"
export RUSTPLAYER_LASTFM_SECRET="your_shared_secret_here"
cargo build --release
```

```powershell
# Windows PowerShell
$env:RUSTPLAYER_LASTFM_KEY="your_api_key_here"
$env:RUSTPLAYER_LASTFM_SECRET="your_shared_secret_here"
cargo build --release
```

5. In RPlayer: open the Audio panel → Tracks tab → Last.fm section
   - Enter your username and password
   - Click "Connect Account"

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
4. Set the environment variable before building (see
   [src/services/opensubtitles.rs](../src/services/opensubtitles.rs)):

```bash
# Linux / macOS
export RUSTPLAYER_OPENSUBS_KEY="your_api_key_here"
cargo build --release
```

```powershell
# Windows PowerShell
$env:RUSTPLAYER_OPENSUBS_KEY="your_api_key_here"
cargo build --release
```

### Use in RPlayer

Open the Video panel → Subtitles tab → type the title → Search → pick a result → Download.

The subtitle is saved next to the video file and loaded automatically.
