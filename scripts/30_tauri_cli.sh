#!/usr/bin/env bash
# 30_tauri_cli.sh ─ cargo-tauri プリビルト展開（数秒）※失敗時はソースビルド
set -euxo pipefail
TAURI_VER="2.5.0"
ARCH="$(uname -m)"                                 # x86_64 / aarch64
LIBC="$(ldd --version 2>&1 | grep -qi musl && echo musl || echo gnu)"
ASSET="cargo-tauri_${TAURI_VER}_Linux_${ARCH}_${LIBC}.tar.gz"
URL="https://github.com/tauri-apps/tauri/releases/download/tauri-cli-v${TAURI_VER}/${ASSET}"

if curl -fsSL "$URL" -o /tmp/tauri.tgz; then
  tar -xzf /tmp/tauri.tgz -C /usr/local/bin cargo-tauri
  chmod +x /usr/local/bin/cargo-tauri
  echo "[30] cargo-tauri binary installed"
else
  echo "[30] prebuilt not found; falling back to cargo install (≈6 分)"
  cargo install tauri-cli --version "^${TAURI_VER%.*}" --locked
fi
