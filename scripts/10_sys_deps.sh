#!/usr/bin/env bash
# 10_sys_deps.sh ─ 必須ライブラリを最小インストール（1-2 分）
set -euxo pipefail
export DEBIAN_FRONTEND=noninteractive
apt-get update -y
apt-get install -y --no-install-recommends \
  libwebkit2gtk-4.1-dev libsoup-3.0-dev libssl-dev \
  libgtk-3-dev build-essential curl git ca-certificates
echo "[10] apt packages installed"
