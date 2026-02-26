---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P1-min1（.hako-only roadmap P1開始）として `--emit-wat` CLI 入口を固定する。
Related:
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/cli/args.rs
  - src/cli/mod.rs
  - src/cli/groups.rs
  - src/runner/dispatch.rs
  - src/runner/modes/wasm.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p1_emit_wat_cli_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-150 WSM-P1-min1 Emit WAT CLI Lock

## Purpose
`.hako`-only WASM 出力移行の P1 を着手するため、WAT出力の独立入口を `--emit-wat` として固定する。

## Decision
1. 新CLI `--emit-wat <FILE>` を追加し、入力ソースから WAT を生成して指定ファイルへ出力後に終了する契約を固定した。
2. `--emit-wat` は `--compile-wasm` / `--compile-native` / `--aot` と同時指定不可にした（境界曖昧化を防止）。
3. dispatch に `emit_wat` ルートを追加し、`compile_wasm` より先に評価するよう固定した。
4. runtime/compiler の内部実装は既存 `compile_to_wat` を再利用し、P1開始時点では挙動不変を維持した。
5. gate:
   - `phase29cc_wsm_p1_emit_wat_cli_vm.sh` を追加
   - `tools/checks/dev_gate.sh wasm-boundary-lite` に emit-wat step を追加

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p1_emit_wat_cli_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P1-min2`: fixture単位の WAT parity 比較（Rust emit と比較）を lock する。
