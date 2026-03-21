---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P10-min5 として loop/extern native promotion の拡張在庫（warn/info/error/debug）を analysis-only で固定し、default route は bridge 契約のまま維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-197-wsm-p10-min4-single-fixture-native-promotion-lock-ssot.md
  - apps/tests/phase29cc_wsm_p10_min5_loop_extern_warn_inventory.hako
  - apps/tests/phase29cc_wsm_p10_min5_loop_extern_info_inventory.hako
  - src/backend/wasm/shape_table.rs
  - src/backend/wasm/mod.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min5_expansion_inventory_lock_vm.sh
  - tools/checks/phase29cc_wsm_p10_expansion_inventory_guard.sh
---

# 29cc-198 WSM-P10-min5 Expansion Inventory Lock

## Purpose
`WSM-P10-min4` で 1 fixture native promotion を固定した次段として、
loop/extern の隣接 shape（warn/info/error/debug）を inventory として固定する。  
この段階では route は変更しない（bridge-only 維持）。

## Contract
1. expansion inventory id を次の4語彙で固定する。
   - `wsm.p10.main_loop_extern_call.warn.fixed3.inventory.v0`
   - `wsm.p10.main_loop_extern_call.info.fixed3.inventory.v0`
   - `wsm.p10.main_loop_extern_call.error.fixed3.inventory.v0`
   - `wsm.p10.main_loop_extern_call.debug.fixed3.inventory.v0`
2. inventory matcher は analysis-only とし、`compile_hako_native_shape_emit` へ接続しない。
3. `apps/tests/phase29cc_wsm_p10_min5_loop_extern_warn_inventory.hako` / `...info...` は native helper で `None` を返す（bridge-only 境界固定）。
4. `WSM-P10-min4` (`wsm.p10.main_loop_extern_call.fixed3.v0`) の native 昇格契約は回帰させない。

## Acceptance
1. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/p10/phase29cc_wsm_p10_min5_expansion_inventory_lock_vm.sh`
2. `bash tools/checks/phase29cc_wsm_p10_expansion_inventory_guard.sh`
3. `tools/checks/dev_gate.sh portability`

## Next
1. `WSM-P10-min6`: expansion inventory から 1 method family（warn）を native promotion へ昇格し、bridge fallback との境界を再固定する。
