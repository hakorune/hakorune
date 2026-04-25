---
Status: Active
Date: 2026-04-25
Scope: Prune the redundant `mir_call` MapBox.has need-policy branch while keeping the required route-state fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-257-mir-call-mapbox-receiver-surface-review-card.md
  - docs/development/current/main/phases/phase-291x/291x-258-mir-call-runtime-data-receiver-surface-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-259 MIR-Call MapBox.has Need Prune Card

## Goal

Continue the `has` family cleanup without removing the `MapBox` route-state
fallback that 291x-257 proved is still required.

## Change

Removed the `mir_call_need_policy` fallback branch:

```text
MapBox + has -> NEED_MAP_HAS
```

The `mir_call_route_policy` fallback remains:

```text
MapBox + has -> RUNTIME_MAP_HAS
```

## Why This Is Safe

291x-257 showed that metadata-absent direct `MapBox.has` still needs
`runtime_map_has` route-state to select the generic has lowering path.

This card checks a narrower question: whether that metadata-absent path also
needs `nyash.map.probe_*` declarations from `NEED_MAP_HAS`.

It does not. With the need branch removed, metadata-absent direct `MapBox.has`
still compiles:

```text
bname=MapBox mname=has ... map_has:1
object written
```

The emitted fallback is the method-specific generic has compatibility path:

```text
plan.runtime_map_has -> nyash.runtime_data.has_hh
```

Map-probe declarations are still emitted for metadata-backed MapHas routes
through the existing CoreMethod metadata need consumer, not through this legacy
method-surface branch.

## Boundary

- Keep `MapBox` receiver-surface route classification.
- Keep `has` method-surface classification.
- Keep generic `mname == "has"` emit-kind fallback.
- Keep `ArrayBox` and `RuntimeDataBox` generic has-route compatibility rows.
- Do not infer a `MapBox` route-state prune from this need-only cleanup.

## Next Work

Continue the has family review:

```text
generic mname == "has"
generic ArrayBox has-route fallback
generic RuntimeDataBox has-route fallback
mir_call method-surface has
```

Current evidence says these rows are still likely pinned by metadata-absent
boundaries, but each row must be judged explicitly.

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
