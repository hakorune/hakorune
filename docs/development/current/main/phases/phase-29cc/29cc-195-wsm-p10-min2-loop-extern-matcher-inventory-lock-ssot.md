---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P10-min2 として loop+extern の candidate matcher を analysis-only で固定し、native route への波及を禁止したまま inventory を確立する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-194-wsm-p10-min1-loop-extern-native-emit-design-lock-ssot.md
  - src/backend/wasm/shape_table.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min2_loop_extern_matcher_inventory_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_loop_extern_matcher_inventory_guard.sh
---

# 29cc-195 WSM-P10-min2 Loop/Extern Matcher Inventory Lock

## Purpose
`WSM-P10-min1` で固定した設計境界に沿って、loop+extern 候補を検出する matcher を追加する。  
この段階では route 変更を行わず、analysis-only で inventory を固定する。

## Contract
1. candidate id は `wsm.p10.main_loop_extern_call.v0` を単一語彙として固定する。
2. candidate 検出は bridge route を変更しない（`plan=bridge-rust-backend` 維持）。
3. matcher は `main` 関数内の `Branch + Jump + Extern Call` を最小条件として扱う。
4. 非loop extern（single block）は candidate として受理しない。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p10_min2_loop_extern_matcher_inventory_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_loop_extern_matcher_inventory_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P10-min3`: loop/branch/call writer section contract lock（label/local/call ABI）。
