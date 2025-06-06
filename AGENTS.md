

## スクリプト実行手順

以下の手順で開発用コンテナ内でセットアップスクリプトを実行してください。キャッシュを活用するため、**/cache** ボリュームをホストにマウントしている前提です。

```bash
# 1. 必須ライブラリのインストール（所要 1–2 分）
bash scripts/10_sys_deps.sh

# 2. Rust ツールチェインのセットアップ（初回のみ 30 秒程度）
bash scripts/20_rust_toolchain.sh

# 3. Tauri CLI プリビルト展開（数秒）
bash scripts/30_tauri_cli.sh

# 4. Node / pnpm の準備（数秒）
bash scripts/40_node_pnpm.sh

# 5. フロントエンド依存の解決（プロジェクト規模による）
bash scripts/50_js_deps.sh
```

> **二回目以降**は `/cache` にキャッシュされた成果物が再利用されるため、各ステップは数秒〜十数秒で完了します。
