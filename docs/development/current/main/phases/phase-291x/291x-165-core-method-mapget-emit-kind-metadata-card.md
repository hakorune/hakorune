---
Status: Landed
Date: 2026-04-24
Scope: Make generic-method `get` emit-kind selection prefer MIR CoreMethod metadata before legacy method-name fallback.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-151-core-method-get-inc-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-162-core-method-maphas-emit-kind-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc
---

# 291x-165 CoreMethod MapGet Emit-Kind Metadata Card

## Goal

Move the `get` emit-kind selector to the same metadata-first boundary as
MapHas:

```text
generic_method_routes[].core_method.op = MapGet
  -> dispatch selects HAKO_LLVMC_GENERIC_METHOD_EMIT_GET
  -> legacy mname == "get" classifier remains fallback only
```

This is a one-family migration card. It does not delete allowlist rows and does
not change `get` lowering or helper symbols.

## Boundary

- Do not add hot inline lowering.
- Do not remove the legacy `mname == "get"` fallback in this card.
- Do not change `nyash.runtime_data.get_hh`, `nyash.map.slot_load_hh`, or
  array get helper selection.
- Do not infer return-shape or publication policy in the emit-kind selector;
  detailed validation remains in the existing `get` policy reader.

## Implementation

- Extend the generic emit-kind metadata selector to accept
  `route_id=generic_method.get` with `core_method.op=MapGet`,
  `proof=core_method_contract_manifest`, and `lowering_tier=cold_fallback`.
- Keep metadata-absent `get` routes on the legacy method-name fallback.

## Result

- `emit_mir_call_dispatch(...)` can now select `EMIT_GET` from valid MIR
  MapGet CoreMethod metadata before the legacy method-name classifier.
- Metadata-absent RuntimeData `get` fixtures still compile through the legacy
  fallback.
- `get` helper symbols, return-shape validation, and publication policy remain
  owned by the existing `get` policy reader.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_routes
env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dev_gate.sh quick
git diff --check
```
