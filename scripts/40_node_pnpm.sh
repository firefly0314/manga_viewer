#!/usr/bin/env bash
# 40_node_pnpm.sh ─ Node ランタイム準備（数秒）
set -euxo pipefail
corepack enable
corepack install -g pnpm@latest                  # 10.x 系
rm -f package-lock.json                          # npm lock があれば削除
echo "[40] pnpm ready"
