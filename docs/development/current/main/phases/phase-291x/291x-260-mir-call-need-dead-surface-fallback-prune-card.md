---
Status: Landed
Date: 2026-04-26
Scope: Remove the dead `mir_call` need-policy method-surface fallback after has need branches were pruned.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-259-mir-call-mapbox-has-need-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc
---

# 291x-260 MIR-Call Need Dead Surface Fallback Prune Card

## Goal

Keep the `mir_call` need prepass thin after 291x-259 removed the last
method-surface need fallback.

## Change

Removed the dead fallback function:

```text
classify_mir_call_method_need_kind(...)
```

and removed the prepass fallback call to it.

## Decision

Need-policy now has one method-call source of truth:

```text
CoreMethod route metadata -> MirCallNeedKind
```

Metadata-absent method calls no longer re-read receiver/method surface names in
the need prepass. This is correct because the remaining metadata-absent `has`
fallbacks emit through `nyash.runtime_data.has_hh`, whose declaration is
unconditional in the pure compile shell. Map probe declarations are still owned
by CoreMethod metadata-backed `MapHas` routes.

## Boundary

- Do not remove `MapBox + has -> RUNTIME_MAP_HAS` route-state fallback.
- Do not remove generic `mname == "has"` emit-kind fallback.
- Do not remove generic `ArrayBox` / `RuntimeDataBox` has-route fallbacks.
- This card changes only need prepass declaration selection.

## Acceptance

```bash
cargo check -q
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
