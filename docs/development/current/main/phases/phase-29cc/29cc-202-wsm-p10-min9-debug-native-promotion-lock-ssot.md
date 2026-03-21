---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P10-min9 として debug family の1 shape（fixed4）を native emit へ昇格し、min5 expansion inventory の bridge-only 境界を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-201-wsm-p10-min8-error-native-promotion-lock-ssot.md
  - apps/tests/phase29cc_wsm_p10_min9_loop_extern_debug_native.hako
  - apps/tests/phase29cc_wsm_p10_min5_loop_extern_debug_inventory.hako
  - src/backend/wasm/shape_table.rs
  - src/backend/wasm/mod.rs
  - src/backend/wasm/binary_writer.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min9_debug_native_promotion_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_debug_native_promotion_guard.sh
---

# 29cc-202 WSM-P10-min9 Debug Native Promotion Lock

## Purpose
`WSM-P10-min8` で固定した warn/info/error 昇格境界を維持したまま、adjacent family の debug を fixed4 で1件だけ native emit へ段階昇格する。  
この段階でも fixed3 inventory（min5）は bridge-only を維持する。

## Contract
1. native promotion shape id は `wsm.p10.main_loop_extern_call.debug.fixed4.v0` を固定する。
2. native writer は `build_loop_extern_call_skeleton_module_with_import(4, LoopExternImport::ConsoleDebug)` を使用する。
3. min5 inventory (`debug.fixed3.inventory.v0`) は bridge-only 維持（native helper は `None`）。
4. route trace は min9 fixture で `shape_id=wsm.p10.main_loop_extern_call.debug.fixed4.v0` を出力する。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min9_debug_native_promotion_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_debug_native_promotion_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P10-min10`: loop/extern native promotion closeout lock（family fixed4 境界の最終固定）。
