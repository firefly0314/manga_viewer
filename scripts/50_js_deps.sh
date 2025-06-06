#!/usr/bin/env bash
# 50_js_deps.sh ─ フロントエンド依存解決（プロジェクト規模による）
set -euxo pipefail
pnpm install --no-frozen-lockfile
# SDK が無ければ追加
if ! pnpm list --depth -1 @tauri-apps/api >/dev/null 2>&1; then
  pnpm add -D @tauri-apps/api@2 @tauri-apps/cli@2
fi
echo "[50] JS dependencies installed"
