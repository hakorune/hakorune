---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P2-min1（.hako-only roadmap P2）として `wat2wasm` bridge 契約を lock する。
Related:
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-151-wsm-p1-min2-wat-parity-lock-ssot.md
  - src/backend/wasm/mod.rs
  - src/runner/modes/wasm.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p2_min1_bridge_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-152 WSM-P2-min1 wat2wasm Bridge Lock

## Purpose
`.hako`-only WASM 出力移行（P2）を進めるため、WAT から `.wasm` への bridge 契約（normal/boundary/error）を fail-fast で固定する。

## Decision
1. `WasmBackend` に `convert_wat_to_wasm` 公開ヘルパを置き、ASCII guard と `wabt::wat2wasm` 失敗伝播を SSOT 化した。
2. `--compile-wasm` ルートは `.wat` ではなく `.wasm` を直接出力するように更新した。
3. `tests/wasm_demo_min_fixture.rs` に bridge 契約テストを追加した。
   - normal: fixture compile-to-wasm
   - normal(cli): `--compile-wasm` が `.wasm` を出力
   - boundary: non-ASCII WAT guard
   - error: malformed WAT fail-fast
4. smoke `phase29cc_wsm_p2_min1_bridge_lock_vm.sh` を追加し、`tools/checks/dev_gate.sh wasm-boundary-lite` へ統合した。

## Acceptance
- `cargo test --features wasm-backend wasm_demo_ -- --nocapture`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p2_min1_bridge_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P3-min1`: JS import object 生成契約（supported list / fail-fast 文言）を `.hako` 側へ移植する。
