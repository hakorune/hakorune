---
Status: Landed
Date: 2026-04-25
Scope: Preflight a MIR-call route/need policy consumer for `generic_method_routes` metadata.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-187-mir-call-set-surface-prune-card.md
  - docs/development/current/main/phases/phase-291x/291x-188-remaining-inc-mirror-inventory-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
---

# 291x-189 MIR-Call Route-Policy Metadata Consumer Preflight Card

## Goal

Define the next structural seam for reducing MIR-call route-policy mirror rows:

```text
MIR generic_method_routes metadata
  -> route policy: GenericMethodRouteState
  -> need policy: GenericPureNeedFlags
```

`291x-187` showed that pruning `classify_mir_call_method_surface(... "set")`
breaks direct Array/Map pure boundaries even though generic-method set
emit/storage metadata consumers already exist.

## Finding

The failure is not only storage-route selection. The same method surface row
feeds `hako_llvmc_ffi_mir_call_need_policy.inc`:

```text
MapBox.set   -> HAKO_LLVMC_MIR_CALL_NEED_MAP_SET
ArrayBox.set -> HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET
```

Without those need flags, pure compile can miss required helper declarations and
fails before normal lowering.

## Required Consumer Shape

The implementation must be two-sided:

```text
read generic_method_routes at block + instruction_index
validate route_id/core_method/proof/lowering_tier
  -> route policy may populate GenericMethodRouteState
  -> need policy may select MirCallNeedKind
fallback to existing box/method-name classifiers when metadata is absent
```

For the first slice, accept only direct set metadata:

```text
route_id = generic_method.set
core_method.op = ArraySet | MapSet
proof = core_method_contract_manifest
lowering_tier = cold_fallback
route_kind = array_store_any | map_store_any
```

Need mapping:

```text
ArraySet -> HAKO_LLVMC_MIR_CALL_NEED_ARRAY_SET
MapSet   -> HAKO_LLVMC_MIR_CALL_NEED_MAP_SET
```

Route-state mapping should stay conservative. Since direct set routes do not
currently need runtime array/map read flags, the first consumer may focus on
need-policy metadata and leave `GenericMethodRouteState` fallback intact.

## Boundary

- Do not remove any mirror row in the implementation card.
- Do not change `RuntimeDataBox.set` metadata-absent fallback.
- Do not change helper symbols or lowering.
- Do not accept mismatched `ArraySet + map_store_any` or
  `MapSet + array_store_any`.
- Do not duplicate proof/lowering-tier parsing in multiple places without a
  shared reader seam. If the include order blocks reuse of
  `hako_llvmc_ffi_core_method_metadata.inc`, move or introduce a small shared
  route metadata reader first.

## Result

The next implementation target is a metadata-first need-policy consumer for
direct `ArraySet`/`MapSet`. A later prune probe may retry the MIR-call `set`
surface row only after that consumer lands and the boundary smokes stay green.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
