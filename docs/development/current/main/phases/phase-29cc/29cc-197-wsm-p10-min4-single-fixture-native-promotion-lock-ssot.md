---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P10-min4 として loop/extern 1 fixture を native emit へ昇格し、既存 bridge fixture 契約を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-196-wsm-p10-min3-loop-extern-writer-section-lock-ssot.md
  - apps/tests/phase29cc_wsm_p10_min4_loop_extern_native.hako
  - src/backend/wasm/shape_table.rs
  - src/backend/wasm/mod.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min4_single_fixture_native_promotion_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_single_fixture_native_promotion_guard.sh
---

# 29cc-197 WSM-P10-min4 Single Fixture Native Promotion Lock

## Purpose
`WSM-P10-min3` で固定した writer section 契約を使い、
loop/extern 1 fixture を native emit へ昇格する。  
この段階では既存 bridge blocker fixture（webcanvas/canvas_advanced）は bridge 契約のまま維持する。

## Contract
1. native promotion 対象は `apps/tests/phase29cc_wsm_p10_min4_loop_extern_native.hako` の1件のみ。
2. native shape id は `wsm.p10.main_loop_extern_call.fixed3.v0` を固定する。
3. native writer は `build_loop_extern_call_skeleton_module(3)` を使用する。
4. webcanvas/canvas_advanced の bridge 契約は変更しない（回帰禁止）。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min4_single_fixture_native_promotion_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_single_fixture_native_promotion_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P10-min5`: loop/extern native promotion の範囲拡張 inventory（bridge契約を壊さない段階拡張）。
