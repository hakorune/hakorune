---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P5-min4（.hako-only roadmap P5）として default(hako-lane) の bridge 依存を 1 shape 縮退し、native/bridge 境界を lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-162-wsm-p5-min3-default-hako-lane-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/backend/wasm/mod.rs
  - src/runner/modes/wasm.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-163 WSM-P5-min4 Hako-Lane Bridge Shrink Lock

## Purpose
`WSM-P5-min3` の default(hako-lane) 切替を前提に、hako-lane 内部の実行計画を明示化する。  
pilot shape では native path、非pilot shape では bridge fallback とし、責務境界を fail-fast 契約で固定する。

## Decision
1. default(hako-lane) は `WasmHakoDefaultLanePlan` を SSOT とする。
2. plan は `NativePilotShape` / `BridgeRustBackend` の2値のみ。
3. pilot shape (`main return const i32`) は native path で wasm bytes を生成する。
4. 非pilot shape は明示 bridge fallback（Rust backend compile path）を使う。

## Implemented
1. `src/backend/wasm/mod.rs`
   - `WasmHakoDefaultLanePlan` を追加。
   - `plan_hako_default_lane(&MirModule)` を追加。
   - `compile_hako_default_lane(MirModule)` を追加（bytes + plan を返す）。
2. `src/runner/modes/wasm.rs`
   - default(hako-lane) は `compile_hako_default_lane` を経由。
3. contracts
   - backend unit:
     - `wasm_hako_default_lane_plan_native_for_pilot_shape_contract`
     - `wasm_hako_default_lane_plan_bridge_for_non_pilot_shape_contract`
   - fixture:
     - `wasm_demo_default_hako_lane_native_pilot_shape_contract`
     - `wasm_demo_default_hako_lane_bridge_non_pilot_contract`

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min4_hako_lane_bridge_shrink_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min5`: `.hako` emitter/binary writer 実体路を 1 shape で接続し、bridge fallback 非依存の case を lock する。
