---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P5-min7（.hako-only roadmap P5）として wasm compile route の shape-id 可観測性（route trace）を固定し、legacy lane 縮退判定の観測証跡を残せるようにする。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-165-wsm-p5-min6-shape-expand-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/reference/environment-variables.md
  - src/backend/wasm/mod.rs
  - src/runner/modes/wasm.rs
  - src/config/env/runner_flags.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-166 WSM-P5-min7 Shape Trace Lock

## Purpose
shape-table native path の採用可否を「どの shape_id で決まったか」まで1行で観測できるようにし、default/legacy policy 判定の証跡を SSOT 化する。

## Decision
1. `NYASH_WASM_ROUTE_TRACE=1` を導入し、WASM route 判定時に1行タグを出す。
2. ログタグは固定: `[wasm/route-trace] policy=<...> plan=<...> shape_id=<...>`。
3. default route は `native-shape-table` / `bridge-rust-backend` のどちらかを出す。
4. legacy policy は `legacy-rust` を出し、shape_id は `-` とする。
5. trace は既定 OFF（診断時のみ ON）。

## Implemented
1. `src/backend/wasm/mod.rs`
   - `WasmNativeShapeEmit` と `compile_hako_native_shape_emit` を追加。
   - `WasmHakoDefaultLaneTrace` と `plan_hako_default_lane_trace` を追加。
2. `src/runner/modes/wasm.rs`
   - `emit_wasm_route_trace` を追加し、compile route ごとに stable line を出力。
3. `src/config/env/runner_flags.rs`
   - `wasm_route_trace_enabled()` を追加（`NYASH_WASM_ROUTE_TRACE`）。
4. docs/catalog
   - `docs/reference/environment-variables.md` と `src/config/env/catalog.rs` に `NYASH_WASM_ROUTE_TRACE` を追加。
   - logging contract SSOT (`ai-handoff-and-debug-contract.md`) にタグ契約を追記。
5. tests/smoke
   - `tests/wasm_demo_min_fixture.rs` に route trace 契約を追加。
   - `tools/smokes/.../phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh` を追加。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min7_shape_trace_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min8`: legacy lane retire readiness（trace 証跡 + gate 統計）を docs-first で固定し、retire 判定基準を lock する。
