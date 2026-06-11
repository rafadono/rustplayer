#!/usr/bin/env bash
# build-release.sh — Builds RustPlayer in release mode with all protections.
#
# Use:
#   ./scripts/build-release.sh
#   ./scripts/build-release.sh --package # also generates .tar.gz and .deb
#
# If you have API keys configured, load the .env first:
#   source .env && ./scripts/build-release.sh

set -euo pipefail

BINARY="rustplayer"
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
TARGET_DIR="target/release"
ARTIFACTS_DIR="artifacts"

echo "=== RustPlayer v${VERSION} — build release ==="

# ── Check system dependencies ──────────────────── ────────────────────
check_dep() {
    if ! command -v "$1" &>/dev/null; then
        echo "ERROR: '$1' no está instalado."
        echo "  Fedora:  sudo dnf install $2"
        echo "  Ubuntu:  sudo apt install $3"
        exit 1
    fi
}

check_dep pkg-config  pkg-config    pkg-config
check_dep cargo       cargo         cargo

# Verify that libmpv is available
if ! pkg-config --exists mpv 2>/dev/null; then
    echo "ERROR: libmpv-dev no encontrado."
    echo "  Fedora:  sudo dnf install mpv-libs-devel"
    echo "  Ubuntu:  sudo apt install libmpv-dev"
    exit 1
fi

# ── Show active API keys (without revealing the value) ───────────────────────────
echo ""
echo "Keys configuradas:"
if [[ -n "${RUSTPLAYER_LASTFM_KEY:-}" ]]; then
    echo "  Last.fm:       ✓ (variable de entorno)"
else
    echo "  Last.fm:       — (usando valor compilado, puede ser placeholder)"
fi
if [[ -n "${RUSTPLAYER_OPENSUBS_KEY:-}" ]]; then
    echo "  OpenSubtitles: ✓ (variable de entorno)"
else
    echo "  OpenSubtitles: — (usando valor compilado, puede ser placeholder)"
fi
echo ""

# ── Compile ───────────────────────────────── ─────────────────────────────────
echo "Compilando..."

# These flags are added to what [profile.release] already has in Cargo.toml
RUSTFLAGS="-C target-cpu=native" \
cargo build --release 2>&1

# Verify that the binary was generated
if [[ ! -f "${TARGET_DIR}/${BINARY}" ]]; then
    echo "ERROR: El binario no se generó."
    exit 1
fi

SIZE=$(du -sh "${TARGET_DIR}/${BINARY}" | cut -f1)
echo ""
echo "✓ Binario generado: ${TARGET_DIR}/${BINARY} (${SIZE})"

# ── Verify that there are no sensitive strings visible ───────────────────────────
echo ""
echo "Verificando strings expuestas..."

LEAKED=0

# Check that the real API keys do not appear in the binary
for key in "${RUSTPLAYER_LASTFM_KEY:-}" "${RUSTPLAYER_OPENSUBS_KEY:-}"; do
    if [[ -n "$key" ]] && strings "${TARGET_DIR}/${BINARY}" | grep -qF "$key" 2>/dev/null; then
        echo "  ADVERTENCIA: Una API key aparece en texto plano en el binario."
        LEAKED=1
    fi
done

# Check that there are no source code paths left (the strip should have removed them)
if strings "${TARGET_DIR}/${BINARY}" | grep -q 'src/lastfm.rs\|src/opensubtitles.rs' 2>/dev/null; then
    echo "  ADVERTENCIA: Paths de código fuente visibles — verifica que strip=true está activo."
    LEAKED=1
fi

if [[ $LEAKED -eq 0 ]]; then
    echo "  ✓ Sin strings sensibles visibles."
fi

# ── Package (optional) ────────────────────────── ───────────────────────────
if [[ "${1:-}" == "--package" ]]; then
    mkdir -p "${ARTIFACTS_DIR}"

    TARBALL="${ARTIFACTS_DIR}/${BINARY}-${VERSION}-linux-x86_64.tar.gz"
    tar -czf "${TARBALL}" \
        -C "${TARGET_DIR}" "${BINARY}" \
        -C "$(pwd)" README.md LICENSE docs/

    echo ""
    echo "✓ Paquete: ${TARBALL} ($(du -sh "${TARBALL}" | cut -f1))"

    # basic .deb if dpkg-deb is available
    if command -v dpkg-deb &>/dev/null; then
        build_deb "${VERSION}"
    fi
fi

echo ""
echo "=== Build completado ==="

# ── Function to generate .deb ──────────────────────── ─────────────────────────
build_deb() {
    local ver="$1"
    local deb_dir="${ARTIFACTS_DIR}/deb/${BINARY}_${ver}_amd64"

    mkdir -p "${deb_dir}/usr/bin"
    mkdir -p "${deb_dir}/usr/share/doc/${BINARY}"
    mkdir -p "${deb_dir}/DEBIAN"

    cp "${TARGET_DIR}/${BINARY}" "${deb_dir}/usr/bin/"
    cp README.md LICENSE "${deb_dir}/usr/share/doc/${BINARY}/"

    cat > "${deb_dir}/DEBIAN/control" << EOF
Package: rustplayer
Version: ${ver}
Architecture: amd64
Maintainer: RustPlayer
Description: Reproductor de video y audio libre
 Reproductor basado en libmpv con interfaz gráfica egui.
Depends: libmpv1 | mpv, ffmpeg
EOF

    dpkg-deb --build "${deb_dir}" "${ARTIFACTS_DIR}/${BINARY}_${ver}_amd64.deb" 2>/dev/null
    echo "✓ .deb: ${ARTIFACTS_DIR}/${BINARY}_${ver}_amd64.deb"
}
