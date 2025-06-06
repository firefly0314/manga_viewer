#!/usr/bin/env bash
# 20_rust_toolchain.sh ─ Rust 1.87.0 をキャッシュにインストール（初回30 秒）
set -euxo pipefail
TOOLCHAIN="1.87.0"
if ! rustup toolchain list | grep -q "$TOOLCHAIN"; then
  rustup toolchain install "$TOOLCHAIN"              # 初回のみ転送あり
fi
rustup default "$TOOLCHAIN"
echo "[20] rustup toolchain ready: $TOOLCHAIN"
