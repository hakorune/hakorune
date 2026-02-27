---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P5-min3（.hako-only roadmap P5）として default route を hako-lane へ切替し、legacy lane 差分 gate を lock する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-161-wsm-p5-min2-route-policy-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/runner/modes/wasm.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min3_default_hako_lane_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-162 WSM-P5-min3 Default Hako-Lane Lock

## Purpose
`WSM-P5-min2` の route policy SSOT を前提に、`default` route を `hako-lane` 名義へ切替する。  
現段階では `.hako` emitter/binary writer の独立実行路が未接続のため、`hako-lane` は bridge 実装で Rust backend へ委譲する。

## Decision
1. route 選択は `default -> hako-lane(bridge)`、`legacy-wasm-rust -> legacy-rust lane` とする。
2. `default` と `legacy` の出力 parity を fixture で lock し、cutover 中の挙動崩れを防止する。
3. silent fallback は禁止し、差分は route policy による明示選択だけで管理する。

## Implemented
1. `src/runner/modes/wasm.rs`
   - compile route 選択関数を追加（`WasmCompileRoute`）。
   - `default` は `HakoDefaultBridge` へ切替。
   - `legacy-wasm-rust` は `LegacyRust` へ固定。
2. route contract tests
   - `wasm_compile_route_policy_default_maps_to_hako_bridge_contract`
   - `wasm_compile_route_policy_legacy_maps_to_legacy_rust_contract`
3. CLI parity fixture
   - `wasm_demo_min_fixture_route_policy_default_vs_legacy_cli_parity_contract`

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min3_default_hako_lane_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min4`: `default(hako-lane)` の bridge 依存を縮退し、`.hako` emitter/binary writer 実体路へ 1 shape ずつ切替する。
