---
Status: Ready
Scope: docs-only (runbook)
---

# Phase 29bg P0: Run hako_check gate（docs-first）

## Goal

`hako_check`（旧称 `hack_check`）を「動くこと」「どのコマンドが gate か」を 1 枚で固定する。

## What is hako_check?

- `.hako` の品質チェックツール（Nyash VM で `tools/hako_check/cli.hako` を実行）
- 入口スクリプト: `./tools/hako_check.sh`

## Preconditions

- `cargo build --release`（`./target/release/hakorune` が必要）

## Commands (recommended order)

1. Build
   - `cargo build --release`

2. Smoke: dead code（HC019）
   - `./tools/hako_check_deadcode_smoke.sh`

3. Smoke: dead blocks（HC020, MVP）
   - `./tools/hako_check_deadblocks_smoke.sh`
   - Note: “CFG info not available” は MVP では許容（スクリプトが 0 で終わることを確認）

4. Analyzer test suite（tools/hako_check/tests）
   - `./tools/hako_check/run_tests.sh`

5. Selfhost runtime quick check（最小）
   - `./tools/hako_check.sh apps/selfhost-runtime/boxes_std.hako`

## Useful variants

- ディレクトリをまとめて:
  - `./tools/hako_check.sh apps/selfhost-runtime/`
- dead-code を実際の tree で:
  - `./tools/hako_check.sh --dead-code apps/selfhost-runtime/`
- LSP向け JSON:
  - `./tools/hako_check.sh --format json-lsp apps/selfhost-runtime/boxes_std.hako`

## Expected results

- `./tools/hako_check.sh ...` は “RC: 0” が最終的に出て、終了コード 0
- 失敗時は `[lint/summary] failures: N` で終了コード 1

## Notes

- `HAKO_CHECK_DEBUG=1` で [DEBUG] ノイズフィルタを無効化できる（普段は 0 推奨）。
- `NYASH_BIN=/path/to/hakorune` で使用バイナリを差し替え可能。

