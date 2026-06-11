//! remote.rs — Local HTTP server for remote control.
//!
//! Endpoints:
//!   GET / → basic control HTML page
//!   GET /status → JSON with current status
//!   POST /play → play/pause toggle
//!   POST /pause → pause
//!   POST /resume → resume
//!   POST /stop → stop
//!   POST /seek?t=120 → go to second 120
//!   POST /volume?v=80→ volume 0-150
//!   POST /next → next track
//!   POST /prev → previous track

use crossbeam_channel::{bounded, Receiver, Sender};
use log::{debug, info};
use std::thread;

#[derive(Debug, Clone)]
pub enum RemoteCommand {
    TogglePause,
    Pause,
    Resume,
    Stop,
    Seek(f64),
    SetVolume(i64),
    Next,
    Prev,
}

pub struct RemoteServer {
    pub cmd_rx: Receiver<RemoteCommand>,
}

impl RemoteServer {
    pub fn start(port: u16) -> Option<Self> {
        let (tx, rx) = bounded::<RemoteCommand>(64);
        let addr = format!("127.0.0.1:{}", port);

        let server = tiny_http::Server::http(&addr).ok()?;
        info!("Control remoto en http://{}", addr);

        thread::Builder::new()
            .name("remote-http".into())
            .spawn(move || {
                for req in server.incoming_requests() {
                    handle(req, &tx);
                }
            })
            .ok()?;

        Some(Self { cmd_rx: rx })
    }

    pub fn drain(&self) -> Vec<RemoteCommand> {
        self.cmd_rx.try_iter().collect()
    }
}

fn handle(req: tiny_http::Request, tx: &Sender<RemoteCommand>) {
    let url = req.url().to_string();
    let method = req.method().as_str().to_uppercase();
    debug!("remote: {} {}", method, url);

    // Parse path and query
    let (path, query) = url
        .split_once('?')
        .map(|(p, q)| (p, q))
        .unwrap_or((&url, ""));

    let resp = match (method.as_str(), path) {
        ("GET", "/") => respond_html(HTML_PAGE),
        ("GET", "/status") => respond_json(r#"{"ok":true}"#),

        ("POST", "/play") => {
            let _ = tx.try_send(RemoteCommand::TogglePause);
            respond_json(r#"{"ok":true}"#)
        }
        ("POST", "/pause") => {
            let _ = tx.try_send(RemoteCommand::Pause);
            respond_json(r#"{"ok":true}"#)
        }
        ("POST", "/resume") => {
            let _ = tx.try_send(RemoteCommand::Resume);
            respond_json(r#"{"ok":true}"#)
        }
        ("POST", "/stop") => {
            let _ = tx.try_send(RemoteCommand::Stop);
            respond_json(r#"{"ok":true}"#)
        }
        ("POST", "/next") => {
            let _ = tx.try_send(RemoteCommand::Next);
            respond_json(r#"{"ok":true}"#)
        }
        ("POST", "/prev") => {
            let _ = tx.try_send(RemoteCommand::Prev);
            respond_json(r#"{"ok":true}"#)
        }
        ("POST", "/seek") => {
            if let Some(t) = get_param(query, "t").and_then(|v| v.parse::<f64>().ok()) {
                let _ = tx.try_send(RemoteCommand::Seek(t));
                respond_json(r#"{"ok":true}"#)
            } else {
                respond_error("falta ?t=segundos")
            }
        }
        ("POST", "/volume") => {
            if let Some(v) = get_param(query, "v").and_then(|v| v.parse::<i64>().ok()) {
                let _ = tx.try_send(RemoteCommand::SetVolume(v.clamp(0, 150)));
                respond_json(r#"{"ok":true}"#)
            } else {
                respond_error("falta ?v=0-150")
            }
        }
        _ => tiny_http::Response::from_string("404").with_status_code(404),
    };

    let _ = req.respond(resp);
}

fn get_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|kv| {
        let (k, v) = kv.split_once('=')?;
        if k == key {
            Some(v)
        } else {
            None
        }
    })
}

fn respond_json(body: &str) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    tiny_http::Response::from_string(body)
        .with_status_code(200)
        .with_header(tiny_http::Header::from_bytes(b"Content-Type", b"application/json").unwrap())
}

fn respond_html(body: &str) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    tiny_http::Response::from_string(body)
        .with_status_code(200)
        .with_header(
            tiny_http::Header::from_bytes(b"Content-Type", b"text/html; charset=utf-8").unwrap(),
        )
}

fn respond_error(msg: &str) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    respond_json(&format!(r#"{{"ok":false,"error":"{}"}}"#, msg))
}

// ── Basic control HTML page ───────────────────── ──────────────────────

const HTML_PAGE: &str = r#"<!DOCTYPE html>
<html lang="es">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>RPlayer — Control Remoto</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { background: #0e0e14; color: #dcdce6; font-family: monospace;
         display: flex; flex-direction: column; align-items: center;
         padding: 2rem; gap: 1.5rem; }
  h1 { font-size: 1.4rem; color: #63b3ed; }
  .brand { display: flex; align-items: center; gap: 10px; }
  .logo {
    width: 34px; height: 34px; border-radius: 8px;
    display: inline-flex; align-items: center; justify-content: center;
    background: linear-gradient(135deg, #63b3ed, #2b6cb0);
    color: #0e0e14; font-weight: 800; letter-spacing: 0.03em;
    border: 1px solid #8bc4f0;
  }
  .grid { display: grid; grid-template-columns: repeat(3, 80px); gap: 10px; }
  button {
    background: #16161f; border: 1px solid #2a2a3a; color: #dcdce6;
    padding: 14px 8px; border-radius: 6px; font-size: 1.2rem; cursor: pointer;
    transition: background 0.15s, border-color 0.15s;
  }
  button:hover { background: #1e1e2a; border-color: #63b3ed; }
  button:active { background: #63b3ed; color: #0e0e14; }
  .vol { display: flex; align-items: center; gap: 10px; width: 260px; }
  input[type=range] { flex: 1; accent-color: #63b3ed; }
  #msg { font-size: 0.8rem; color: #64648f; }
</style>
</head>
<body>
<div class="brand">
  <span class="logo">RP</span>
  <h1>RPlayer</h1>
</div>
<div class="grid">
  <button onclick="cmd('/prev')">⏮</button>
  <button onclick="cmd('/play')" style="font-size:1.6rem">▶</button>
  <button onclick="cmd('/next')">⏭</button>
  <button onclick="seek(-30)">-30s</button>
  <button onclick="cmd('/stop')">⏹</button>
  <button onclick="seek(30)">+30s</button>
</div>
<div class="vol">
  <span>🔈</span>
  <input type="range" min="0" max="150" value="100" id="vol" oninput="setVol(this.value)">
  <span>🔊</span>
</div>
<div id="msg">listo</div>
<script>
async function cmd(endpoint) {
  try {
    await fetch(endpoint, { method: 'POST' });
    document.getElementById('msg').textContent = endpoint;
  } catch(e) { document.getElementById('msg').textContent = 'error: ' + e; }
}
function seek(secs) { cmd('/seek?t=' + secs); }
function setVol(v) { cmd('/volume?v=' + v); }
</script>
</body>
</html>"#;
