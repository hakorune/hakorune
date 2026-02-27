---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P5-min6（.hako-only roadmap P5）として default(hako-lane) の native shape-table を 1 shape 拡張し、bridge fallback 範囲をさらに縮退する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-164-wsm-p5-min5-native-helper-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - src/backend/wasm/shape_table.rs
  - src/backend/wasm/mod.rs
  - src/runner/modes/wasm.rs
  - tests/wasm_demo_min_fixture.rs
  - apps/tests/phase29cc_wsm_p5_min6_const_copy_return.hako
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min6_shape_expand_lock_vm.sh
  - tools/checks/dev_gate.sh
---

# 29cc-165 WSM-P5-min6 Shape Expand Lock

## Purpose
default(hako-lane) の native helper 対象を pilot 1shape から shape-table 2shape へ拡張し、`const->copy->return` を bridge 非依存で固定する。

## Decision
1. native helper を `compile_hako_native_shape_bytes(&MirModule)` に一般化する。
2. shape table は `main return const` に加えて `const->copy->return` を受理する。
3. default(hako-lane) は shape-table helper first を維持し、非該当時のみ bridge fallback を使う。
4. helper が `Err` の場合は fail-fast（silent fallback しない）。

## Implemented
1. `src/backend/wasm/shape_table.rs`
   - shape table に `wsm.p5.main_return_i32_const_via_copy.v0` を追加。
2. `src/backend/wasm/mod.rs`
   - helper を `compile_hako_native_shape_bytes` へ切替。
   - default lane plan を `NativeShapeTable` / `BridgeRustBackend` に更新。
   - unit tests を shape-table 契約へ更新。
3. `src/runner/modes/wasm.rs`
   - default(hako-lane) route は `compile_hako_native_shape_bytes` first へ更新。
4. `tests/wasm_demo_min_fixture.rs` + fixture
   - `apps/tests/phase29cc_wsm_p5_min6_const_copy_return.hako` を追加。
   - const-copy fixture が native helper 経由になる契約テストを追加。

## Acceptance
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p5_min6_shape_expand_lock_vm.sh`
- `tools/checks/dev_gate.sh wasm-boundary-lite`

## Next
- `WSM-P5-min7`: shape-table native path の shape-id 可観測性（route trace）を lock し、legacy lane 縮退判定の観測契約を固定する。
