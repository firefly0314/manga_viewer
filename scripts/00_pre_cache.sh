#!/usr/bin/env bash
# 00_pre_cache.sh ─ キャッシュ用ディレクトリの準備（10 秒未満）
set -eu
: "${CACHE_DIR:=/cache}"                   # docker -v /host/cache:/cache でバインド
export CARGO_TARGET_DIR="${CACHE_DIR}/target"
export RUSTUP_HOME="${CACHE_DIR}/rustup"
mkdir -p "$CARGO_TARGET_DIR" "$RUSTUP_HOME"
echo "[00] cache dirs prepared → $CACHE_DIR"
