---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P5-min5（.hako-only roadmap P5）として default(hako-lane) の 1-shape native helper 実体路を固定し、bridge fallback 非依存 case を lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-163-wsm-p5-min4-hako-lane-bridge-shrink-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/backend/wasm/mod.rs
  - src/runner/modes/wasm.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min5_native_helper_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-164 WSM-P5-min5 Native Helper Lock

## Purpose
default(hako-lane) の pilot 1shape について、bridge 経路に依存しない native helper 実体路を固定する。  
非pilot shape は引き続き bridge fallback として明示し、境界を曖昧にしない。

## Decision
1. pilot native helper を `compile_hako_native_pilot_bytes(&MirModule)` として公開する。
2. runner default(hako-lane) は helper を先に試行し、`Some(bytes)` なら即返す。
3. helper が `None` の場合のみ bridge fallback（Rust backend compile path）へ委譲する。
4. helper 失敗は fail-fast（`Err`）で停止し、silent fallback はしない。

## Implemented
1. `src/backend/wasm/mod.rs`
   - `compile_hako_native_pilot_bytes` 追加。
   - contract tests:
     - `wasm_hako_native_pilot_bytes_emits_for_pilot_shape_contract`
     - `wasm_hako_native_pilot_bytes_rejects_non_pilot_contract`
2. `src/runner/modes/wasm.rs`
   - default(hako-lane) は `compile_hako_native_pilot_bytes` first に変更。
3. `tests/wasm_demo_min_fixture.rs`
   - `wasm_demo_default_route_pilot_uses_native_helper_contract`
   - `wasm_demo_default_route_native_helper_rejects_non_pilot_contract`

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min5_native_helper_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min6`: `.hako` emitter/binary writer 実体路を pilot shape 以外へ 1 shape 拡張し、fallback 範囲をさらに縮退する。
