---
Status: Landed
Date: 2026-04-24
Scope: Make the generic-method `get` `.inc` consumer prefer MIR CoreMethod MapGet metadata.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-136-core-method-has-inc-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-142-mapget-return-shape-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_lowering.inc
---

# 291x-151 CoreMethod `get` `.inc` Consumer Card

## Goal

Move the generic-method `get` C shim one step closer to enum/table consumption:

```text
MIR generic_method_routes.core_method.op = MapGet
  -> .inc consumes MapGet metadata first
  -> legacy bname/plan classifier remains fallback only
```

This card does not add a new lowering. `RuntimeDataBox.get` MapBox-origin
routes still call `nyash.runtime_data.get_hh`; the change is the authority
boundary.

## Design

The `get` consumer mirrors the already-landed `has` metadata path:

```text
generic_method_routes[]
  route_id = generic_method.get
  core_method.op = MapGet
  core_method.proof = core_method_contract_manifest
  core_method.lowering_tier = cold_fallback
```

The `.inc` reader validates the route contract and returns the route enum. It
accepts both conservative mixed returns and scalar-proof metadata, but still
keeps both on the cold RuntimeData facade:

```text
mixed_runtime_i64_or_handle + runtime_i64_or_handle + runtime_data_facade
scalar_i64_or_missing_zero + scalar_i64 + no_publication
```

## Boundary

- Do not change MapGet codegen in this card.
- Do not introduce `slot_load_hi` or scalar load promotion.
- Do not re-prove return shape in `.inc`; only validate MIR-owned metadata.
- Keep direct `bname` classification as the legacy fallback for routes without
  metadata.
- Propagate malformed metadata as fail-fast from the `get` lowering wrapper.

## Implementation

- Added `match_generic_method_get_route_metadata(...)` to the generic-method
  `get` policy seam.
- Added a shared cold-fallback predicate to
  `hako_llvmc_ffi_core_method_metadata.inc`.
- Kept `RuntimeDataBox.get` MapBox-origin lowering on
  `nyash.runtime_data.get_hh`.
- Fixed the `get` lowering wrapper to propagate negative fail-fast returns from
  the policy fallback.

## Acceptance

- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo test -q generic_method_routes`
- `env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh`
- `bash tools/checks/inc_codegen_thin_shim_guard.sh`
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh`
- `bash tools/checks/dev_gate.sh quick`
