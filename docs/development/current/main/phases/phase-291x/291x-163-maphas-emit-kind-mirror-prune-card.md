---
Status: Rejected
Date: 2026-04-24
Scope: Prune the generic-method `has` emit-kind method-name mirror after MapHas metadata-first dispatch.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-162-core-method-maphas-emit-kind-metadata-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-163 MapHas Emit-Kind Mirror Prune Card

## Goal

Remove the generic-method `mname == "has"` emit-kind classifier row now that
dispatch can select `EMIT_HAS` from valid MIR-owned MapHas CoreMethod metadata:

```text
generic_method_routes[].core_method.op = MapHas
  -> EMIT_HAS
legacy classify_generic_method_emit_kind(..., mname="has")
  -> removed for this family
```

This card prunes only the generic emit-kind mirror row. It leaves the mir-call
route-policy `has` surface classifier untouched.

## Boundary

- Do not change `MapHas` lowering or helper symbols.
- Do not prune route-policy `classify_mir_call_method_surface(..., "has")`.
- Do not change `get`, `set`, `len`, `push`, or `substring` classifiers.
- Metadata-absent generic `has` emit-kind was the risk under test; the
  rejection below keeps the old method-name fallback.

## Result

Rejected. The generic `mname == "has"` fallback is still live because
metadata-absent boundary MIR JSON fixtures rely on it.

The attempted prune reduced the no-growth guard to
`classifiers=26 rows=26`, but both RuntimeData boundary smokes failed before
codegen:

```text
phase29ck_boundary_pure_runtime_data_map_has_min
phase29ck_boundary_pure_runtime_data_array_has_min
```

The trace showed `RuntimeDataBox.has` with no usable CoreMethod route metadata,
followed by `unsupported_pure_shape`. Therefore the row is not removable until
those boundary fixtures either carry MIR-owned metadata or a separate
metadata-absent compatibility contract replaces the method-name fallback.

Final state after rejection:

```text
core_method_contract_inc_no_growth_guard: classifiers=27 rows=27
generic_method_policy.inc mname == "has": kept
route_policy has surface row: kept
```

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q generic_method_routes
env NYASH_LLVM_SKIP_BUILD=1 bash tools/smokes/v2/profiles/integration/apps/phase291x_maplookup_fusion_const_fold_contract_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dev_gate.sh quick
git diff --check
```
