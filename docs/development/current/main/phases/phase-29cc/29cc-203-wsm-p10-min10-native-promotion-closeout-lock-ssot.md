---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P10-min10 として loop/extern native promotion family（warn/info/error/debug fixed4）の closeout を固定し、fixed3 inventory の bridge-only 境界を最終ロックする。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-202-wsm-p10-min9-debug-native-promotion-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min10_native_promotion_closeout_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_native_promotion_closeout_guard.sh
  - tools/checks/dev_gate.sh
---

# 29cc-203 WSM-P10-min10 Native Promotion Closeout Lock

## Purpose
`WSM-P10-min6..min9` で段階昇格した warn/info/error/debug fixed4 shape を 1 本の closeout lock として固定する。  
同時に min5 inventory（fixed3）を bridge-only の境界として維持し、P10 を完了状態へ閉じる。

## Contract
1. fixed4 native shape id は次の 4 つのみを許可する。
   - `wsm.p10.main_loop_extern_call.warn.fixed4.v0`
   - `wsm.p10.main_loop_extern_call.info.fixed4.v0`
   - `wsm.p10.main_loop_extern_call.error.fixed4.v0`
   - `wsm.p10.main_loop_extern_call.debug.fixed4.v0`
2. fixed4 native emit は `build_loop_extern_call_skeleton_module_with_import(4, LoopExternImport::...)` で行う。
3. min5 inventory（`warn.fixed3.inventory.v0` / `info.fixed3.inventory.v0` / `error.fixed3.inventory.v0` / `debug.fixed3.inventory.v0`）は native helper で `None` を返し bridge-only を維持する。
4. closeout smoke は min6/min7/min8/min9 lock smoke を束ねて実行し、family 契約をまとめて固定する。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min10_native_promotion_closeout_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_native_promotion_closeout_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. wasm lane active next は `none`（P10 closeout complete; monitor-only）へ更新する。
