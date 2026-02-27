---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P10-min7 として info family の1 shape（fixed4）を native emit へ昇格し、min5 expansion inventory の bridge-only 境界を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-199-wsm-p10-min6-warn-native-promotion-lock-ssot.md
  - apps/tests/phase29cc_wsm_p10_min7_loop_extern_info_native.hako
  - src/backend/wasm/shape_table.rs
  - src/backend/wasm/mod.rs
  - src/backend/wasm/binary_writer.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min7_info_native_promotion_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_info_native_promotion_guard.sh
---

# 29cc-200 WSM-P10-min7 Info Native Promotion Lock

## Purpose
`WSM-P10-min6` で固定した warn 昇格境界を維持したまま、adjacent family の info を fixed4 で1件だけ native emit へ段階昇格する。  
この段階でも fixed3 inventory（min5）は bridge-only を維持する。

## Contract
1. native promotion shape id は `wsm.p10.main_loop_extern_call.info.fixed4.v0` を固定する。
2. native writer は `build_loop_extern_call_skeleton_module_with_import(4, LoopExternImport::ConsoleInfo)` を使用する。
3. min5 inventory (`info.fixed3.inventory.v0`) は bridge-only 維持（native helper は `None`）。
4. route trace は min7 fixture で `shape_id=wsm.p10.main_loop_extern_call.info.fixed4.v0` を出力する。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min7_info_native_promotion_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_info_native_promotion_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P10-min8`: error family native promotion lock（warn/info fixed4 境界を維持した段階拡張）。
