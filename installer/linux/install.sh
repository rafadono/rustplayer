#!/usr/bin/env bash
# install.sh — Installation script for RPlayer on Fedora / Linux
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_SOURCE="${SCRIPT_DIR}/rplayer"

if [[ ! -f "${BIN_SOURCE}" ]]; then
    if [[ -f "${SCRIPT_DIR}/../../target/release/rplayer" ]]; then
        BIN_SOURCE="${SCRIPT_DIR}/../../target/release/rplayer"
    elif [[ -f "${SCRIPT_DIR}/../target/release/rplayer" ]]; then
        BIN_SOURCE="${SCRIPT_DIR}/../target/release/rplayer"
    fi
fi

if [[ ! -f "${BIN_SOURCE}" ]]; then
    echo "ERROR: Could not find the 'rplayer' executable. Please build it first with 'cargo build --release'."
    exit 1
fi

MODE="${1:-user}"

if [[ "${MODE}" == "--system" || "${MODE}" == "system" ]]; then
    echo "=== Installing RPlayer system-wide (/usr/local) ==="
    sudo mkdir -p /usr/local/bin
    sudo mkdir -p /usr/share/applications
    sudo mkdir -p /usr/share/icons/hicolor/256x256/apps

    sudo cp "${BIN_SOURCE}" /usr/local/bin/rplayer
    sudo chmod +x /usr/local/bin/rplayer

    if [[ -f "${SCRIPT_DIR}/rplayer.desktop" ]]; then
        sudo cp "${SCRIPT_DIR}/rplayer.desktop" /usr/share/applications/rplayer.desktop
    fi

    if [[ -f "${SCRIPT_DIR}/assets/icon-rp.png" ]]; then
        sudo cp "${SCRIPT_DIR}/assets/icon-rp.png" /usr/share/icons/hicolor/256x256/apps/rplayer.png
    elif [[ -f "${SCRIPT_DIR}/../../assets/icon-rp.png" ]]; then
        sudo cp "${SCRIPT_DIR}/../../assets/icon-rp.png" /usr/share/icons/hicolor/256x256/apps/rplayer.png
    fi

    if command -v gtk-update-icon-cache &>/dev/null; then
        sudo gtk-update-icon-cache -f -t /usr/share/icons/hicolor 2>/dev/null || true
    fi

    echo "✓ System install complete."
    echo "  Executable: /usr/local/bin/rplayer"
    echo "  Launcher: /usr/share/applications/rplayer.desktop"
else
    echo "=== Installing RPlayer for the current user (~/.local) ==="
    BIN_DIR="${HOME}/.local/bin"
    APP_DIR="${HOME}/.local/share/applications"
    ICON_DIR="${HOME}/.local/share/icons/hicolor/256x256/apps"

    mkdir -p "${BIN_DIR}" "${APP_DIR}" "${ICON_DIR}"

    cp "${BIN_SOURCE}" "${BIN_DIR}/rplayer"
    chmod +x "${BIN_DIR}/rplayer"

    if [[ -f "${SCRIPT_DIR}/rplayer.desktop" ]]; then
        cp "${SCRIPT_DIR}/rplayer.desktop" "${APP_DIR}/rplayer.desktop"
    fi

    ICON_SOURCE=""
    if [[ -f "${SCRIPT_DIR}/assets/icon-rp.png" ]]; then
        ICON_SOURCE="${SCRIPT_DIR}/assets/icon-rp.png"
    elif [[ -f "${SCRIPT_DIR}/../../assets/icon-rp.png" ]]; then
        ICON_SOURCE="${SCRIPT_DIR}/../../assets/icon-rp.png"
    fi

    if [[ -n "${ICON_SOURCE}" ]]; then
        cp "${ICON_SOURCE}" "${ICON_DIR}/rplayer.png"
    fi

    if command -v gtk-update-icon-cache &>/dev/null; then
        gtk-update-icon-cache -f -t "${HOME}/.local/share/icons/hicolor" 2>/dev/null || true
    fi

    echo "✓ User install complete."
    echo "  Executable: ${BIN_DIR}/rplayer"
    echo "  Launcher: ${APP_DIR}/rplayer.desktop"
fi
