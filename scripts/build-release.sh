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

BINARY="rplayer"
VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
TARGET_DIR="target/release"
ARTIFACTS_DIR="artifacts"

echo "=== RustPlayer v${VERSION} — build release ==="

# ── Check system dependencies ──────────────────── ────────────────────
check_dep() {
    if ! command -v "$1" &>/dev/null; then
        echo "ERROR: '$1' is not installed."
        echo "  Fedora:  sudo dnf install $2"
        echo "  Ubuntu:  sudo apt install $3"
        exit 1
    fi
}

check_dep pkg-config  pkg-config    pkg-config
check_dep cargo       cargo         cargo

# Verify that libmpv is available
if ! pkg-config --exists mpv 2>/dev/null; then
    echo "ERROR: libmpv-dev not found."
    echo "  Fedora:  sudo dnf install mpv-libs-devel"
    echo "  Ubuntu:  sudo apt install libmpv-dev"
    exit 1
fi

# ── Show active API keys (without revealing the value) ───────────────────────────
echo ""
echo "Configured keys:"
if [[ -n "${RUSTPLAYER_LASTFM_KEY:-}" ]]; then
    echo "  Last.fm:       ✓ (environment variable)"
else
    echo "  Last.fm:       — (using compiled value, may be a placeholder)"
fi
if [[ -n "${RUSTPLAYER_OPENSUBS_KEY:-}" ]]; then
    echo "  OpenSubtitles: ✓ (environment variable)"
else
    echo "  OpenSubtitles: — (using compiled value, may be a placeholder)"
fi
echo ""

# ── Compile ───────────────────────────────── ─────────────────────────────────
echo "Building..."

# These flags are added to what [profile.release] already has in Cargo.toml
RUSTFLAGS="-C target-cpu=native" \
cargo build --release 2>&1

# Verify that the binary was generated
if [[ ! -f "${TARGET_DIR}/${BINARY}" ]]; then
    echo "ERROR: The binary was not generated."
    exit 1
fi

SIZE=$(du -sh "${TARGET_DIR}/${BINARY}" | cut -f1)
echo ""
echo "✓ Binary generated: ${TARGET_DIR}/${BINARY} (${SIZE})"

# ── Verify that there are no sensitive strings visible ───────────────────────────
echo ""
echo "Checking for exposed strings..."

LEAKED=0

# Check that the real API keys do not appear in the binary
for key in "${RUSTPLAYER_LASTFM_KEY:-}" "${RUSTPLAYER_OPENSUBS_KEY:-}"; do
    if [[ -n "$key" ]] && strings "${TARGET_DIR}/${BINARY}" | grep -qF "$key" 2>/dev/null; then
        echo "  WARNING: An API key appears in plain text in the binary."
        LEAKED=1
    fi
done

# Check that there are no source code paths left (the strip should have removed them)
if strings "${TARGET_DIR}/${BINARY}" | grep -q 'src/lastfm.rs\|src/opensubtitles.rs' 2>/dev/null; then
    echo "  WARNING: Source code paths are visible — verify that strip=true is active."
    LEAKED=1
fi

if [[ $LEAKED -eq 0 ]]; then
    echo "  ✓ No sensitive strings visible."
fi

# ── Function to generate .deb ──────────────────────── ─────────────────────────
build_deb() {
    local ver="$1"
    local deb_dir="${ARTIFACTS_DIR}/deb/${BINARY}_${ver}_amd64"

    mkdir -p "${deb_dir}/usr/bin"
    mkdir -p "${deb_dir}/usr/share/applications"
    mkdir -p "${deb_dir}/usr/share/icons/hicolor/256x256/apps"
    mkdir -p "${deb_dir}/usr/share/doc/${BINARY}"
    mkdir -p "${deb_dir}/DEBIAN"

    cp "${TARGET_DIR}/${BINARY}" "${deb_dir}/usr/bin/"
    cp installer/linux/rplayer.desktop "${deb_dir}/usr/share/applications/"
    cp assets/icon-rp.png "${deb_dir}/usr/share/icons/hicolor/256x256/apps/rplayer.png"
    cp README.md LICENSE "${deb_dir}/usr/share/doc/${BINARY}/"

    cat > "${deb_dir}/DEBIAN/control" << EOF
Package: rplayer
Version: ${ver}
Architecture: amd64
Maintainer: RustPlayer
Description: Free video and audio player
 A libmpv-based player with a Dioxus Desktop GUI.
Depends: libmpv-dev | libmpv2 | libmpv1 | mpv, ffmpeg
EOF

    dpkg-deb --build "${deb_dir}" "${ARTIFACTS_DIR}/${BINARY}_${ver}_amd64.deb" 2>/dev/null
    echo "✓ .deb: ${ARTIFACTS_DIR}/${BINARY}_${ver}_amd64.deb"
}

# ── Function to generate .rpm ──────────────────────── ─────────────────────────
build_rpm() {
    local ver="$(echo "$1" | tr '-' '_')"
    local rpm_root="${ARTIFACTS_DIR}/rpmbuild"

    mkdir -p "${rpm_root}"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

    cat > "${rpm_root}/SPECS/rplayer.spec" << EOF
Name:           rplayer
Version:        ${ver}
Release:        1%{?dist}
Summary:        Free video and audio player
License:        MIT
URL:            https://github.com/rustplayer/rustplayer
Requires:       mpv-libs, ffmpeg

%description
Free video and audio player built in Rust with Dioxus Desktop and libmpv.

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps

install -m 0755 $(pwd)/${TARGET_DIR}/${BINARY} %{buildroot}/usr/bin/rplayer
install -m 0644 $(pwd)/installer/linux/rplayer.desktop %{buildroot}/usr/share/applications/rplayer.desktop
install -m 0644 $(pwd)/assets/icon-rp.png %{buildroot}/usr/share/icons/hicolor/256x256/apps/rplayer.png

%files
/usr/bin/rplayer
/usr/share/applications/rplayer.desktop
/usr/share/icons/hicolor/256x256/apps/rplayer.png

EOF

    rpmbuild --define "_topdir $(pwd)/${rpm_root}" -bb "${rpm_root}/SPECS/rplayer.spec" &>/dev/null || true
    local generated_rpm=$(find "${rpm_root}/RPMS" -name "*.rpm" 2>/dev/null | head -n 1)
    if [[ -n "${generated_rpm}" ]]; then
        cp "${generated_rpm}" "${ARTIFACTS_DIR}/"
        echo "✓ .rpm: ${ARTIFACTS_DIR}/$(basename "${generated_rpm}")"
    fi
    rm -rf "${rpm_root}"
}

# ── Package (optional) ────────────────────────── ───────────────────────────
if [[ "${1:-}" == "--package" ]]; then
    mkdir -p "${ARTIFACTS_DIR}"

    PKG_STAGING="${ARTIFACTS_DIR}/rplayer-${VERSION}-linux-x86_64"
    rm -rf "${PKG_STAGING}"
    mkdir -p "${PKG_STAGING}/assets"

    cp "${TARGET_DIR}/${BINARY}" "${PKG_STAGING}/"
    cp installer/linux/install.sh "${PKG_STAGING}/"
    cp installer/linux/rplayer.desktop "${PKG_STAGING}/"
    cp assets/icon-rp.png "${PKG_STAGING}/assets/"
    cp README.md LICENSE "${PKG_STAGING}/"
    cp -r docs "${PKG_STAGING}/"
    chmod +x "${PKG_STAGING}/install.sh" "${PKG_STAGING}/${BINARY}"

    TARBALL="${ARTIFACTS_DIR}/${BINARY}-${VERSION}-linux-x86_64.tar.gz"
    tar -czf "${TARBALL}" -C "${ARTIFACTS_DIR}" "rplayer-${VERSION}-linux-x86_64"
    rm -rf "${PKG_STAGING}"

    echo ""
    echo "✓ Package generated: ${TARBALL} ($(du -sh "${TARBALL}" | cut -f1))"

    # basic .deb if dpkg-deb is available
    if command -v dpkg-deb &>/dev/null; then
        build_deb "${VERSION}"
    fi

    # basic .rpm if rpmbuild is available
    if command -v rpmbuild &>/dev/null; then
        build_rpm "${VERSION}"
    fi
fi

echo ""
echo "=== Build complete ==="
