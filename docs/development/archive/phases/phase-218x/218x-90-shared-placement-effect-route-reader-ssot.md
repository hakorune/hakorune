# 218x-90 Shared Placement Effect Route Reader SSOT

Status: Landed

Goal
- reduce family-specific C shim folded-route parsing on the `generic placement / effect` lane

Why this cut
- `phase216x` and `phase217x` moved boundary sum and user-box helpers to `placement_effect_routes` first
- the C shim still duplicated folded-route array lookup and route matching across `common` and `sum_local_seed_metadata_helpers`
- that duplication is BoxShape debt inside the active lane

Scope
- add one shared folded-route reader in `hako_llvmc_ffi_common.inc`
- add one shared folded-route matcher in `hako_llvmc_ffi_common.inc`
- reuse those helpers from `hako_llvmc_ffi_sum_local_seed_metadata_helpers.inc`

Non-goals
- removing legacy metadata fallbacks
- changing route semantics
- widening `placement_effect_routes` payload
- string lane migration

Exit
- current boundary sum and user-box helpers consume the same shared folded-route seam
- existing boundary smokes stay green
