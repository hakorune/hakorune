---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P1-min2（.hako-only roadmap P1）として fixture単位 WAT parity を lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-150-wsm-p1-min1-emit-wat-cli-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/backend/wasm/codegen/mod.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p1_parity_wat_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-151 WSM-P1-min2 WAT Parity Lock

## Purpose
`.hako`-only WASM 出力移行（P1）を進めるため、同一 fixture の WAT 出力が Rust基準経路 (`compile_to_wat`) と CLI経路 (`--emit-wat`) で一致する契約を固定する。

## Decision
1. WAT出力安定化のため、WASM codegen の関数生成順と data segment 出力順を決定化した。
2. `tests/wasm_demo_min_fixture.rs` に parity contract（direct vs `--emit-wat`）を追加し、厳密文字列一致を固定した。
3. parity smoke `phase29cc_wsm_p1_parity_wat_vm.sh` を追加し、`tools/checks/dev_gate.sh wasm-boundary-lite` へ組み込んだ。

## Acceptance
- `cargo test --features wasm-backend wasm_demo_min_fixture_emit_wat_parity_contract -- --nocapture`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p1_parity_wat_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P2-min1`: `.hako` 生成 WAT を `wat2wasm` 連結で `.wasm` 化する bridge 契約を lock する。
