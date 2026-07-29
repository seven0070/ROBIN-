#!/usr/bin/env bash
set -euo pipefail

echo "╔══════════════════════════════════════════════╗"
echo "║      Robin Bare-Metal Builder & Installer    ║"
echo "╚══════════════════════════════════════════════╝"

TARGET="${1:-}"

usage() {
    echo "Usage:"
    echo "  ./install.sh              Compile Carry OS boot image only"
    echo "  ./install.sh build        Same as above"
    echo "  ./install.sh /dev/sdX     Build and flash to pendrive"
    exit 1
}

if [ "${TARGET}" = "-h" ] || [ "${TARGET}" = "--help" ]; then
    usage
fi

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT"

echo "[1/4] Compiling Aergon Kernel and Carry OS..."
# Compiles directly to bare-metal x86_64 (no Linux underneath)
cargo bootimage --manifest-path carry/Cargo.toml

IMG_PATH="target/x86_64-unknown-none/debug/bootimage-carry.bin"
if [ ! -f "$IMG_PATH" ]; then
    IMG_PATH="$(find target -name 'bootimage-carry.bin' -type f 2>/dev/null | head -n 1 || true)"
fi

echo "[2/4] Verifying OS Image..."
if [ -z "${IMG_PATH}" ] || [ ! -f "$IMG_PATH" ]; then
    echo "Build failed. Boot image not found."
    exit 1
fi
echo "    Image: $IMG_PATH ($(wc -c < "$IMG_PATH") bytes)"

# Build-only when no device (or explicit "build") is given
if [ -z "$TARGET" ] || [ "$TARGET" = "build" ]; then
    echo "──────────────────────────────────────────────"
    echo "✔ Boot image built (not flashed)."
    echo "  Run: ./install.sh /dev/sdX"
    exit 0
fi

if [ ! -b "$TARGET" ]; then
    echo "Error: $TARGET is not a block device."
    exit 1
fi

echo "[3/4] Preparing Pendrive ($TARGET)..."
echo "WARNING: ALL DATA ON $TARGET WILL BE DESTROYED."
read -r -p "Type YES to flash Robin OS to the pendrive: " CONFIRM
if [ "$CONFIRM" != "YES" ]; then
    echo "Aborted."
    exit 1
fi

echo "[4/4] Flashing Robin to Pendrive..."
sudo dd if="$IMG_PATH" of="$TARGET" bs=4M status=progress conv=fsync
sync

echo "──────────────────────────────────────────────"
echo "✔ Robin is installed."
echo "Eject the USB. Plug it in. Boot from USB."
echo "You are booting a custom OS written from scratch."
