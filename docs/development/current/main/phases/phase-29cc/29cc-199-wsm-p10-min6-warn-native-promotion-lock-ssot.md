---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P10-min6 として warn family の1 shape（fixed4）を native emit へ昇格し、min5 expansion inventory の bridge-only 境界を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-198-wsm-p10-min5-expansion-inventory-lock-ssot.md
  - apps/tests/phase29cc_wsm_p10_min6_loop_extern_warn_native.hako
  - src/backend/wasm/shape_table.rs
  - src/backend/wasm/mod.rs
  - src/backend/wasm/binary_writer.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min6_warn_native_promotion_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_warn_native_promotion_guard.sh
---

# 29cc-199 WSM-P10-min6 Warn Native Promotion Lock

## Purpose
`WSM-P10-min5` で固定した expansion inventory から、warn family の 1 shape だけを native emit へ段階昇格する。  
この段階では fixed3 inventory（min5）を維持し、min6 は fixed4 のみ昇格対象とする。

## Contract
1. native promotion shape id は `wsm.p10.main_loop_extern_call.warn.fixed4.v0` を固定する。
2. native writer は `build_loop_extern_call_skeleton_module_with_import(4, LoopExternImport::ConsoleWarn)` を使用する。
3. min5 inventory (`warn.fixed3.inventory.v0`) は bridge-only 維持（native helper は `None`）。
4. route trace は min6 fixture で `shape_id=wsm.p10.main_loop_extern_call.warn.fixed4.v0` を出力する。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min6_warn_native_promotion_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_warn_native_promotion_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P10-min7`: info family native promotion lock（warn fixed4 境界を維持した段階拡張）。
