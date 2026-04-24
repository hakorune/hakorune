---
Status: Landed
Date: 2026-04-25
Scope: Prune MIR-call route-policy `len`/`length`/`size` method-surface mirrors after length metadata coverage landed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-207-len-emit-kind-mirror-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-208 Len Route-Surface Mirror Prune Card

## Goal

Remove the MIR-call route-policy length alias classifier:

```c
if (!strcmp(mname, "size") || !strcmp(mname, "len") || !strcmp(mname, "length")) {
  return HAKO_LLVMC_MIR_CALL_METHOD_SURFACE_SIZE;
}
```

Length route-state should come from `generic_method_routes` CoreMethod metadata.

## Boundary

- Remove only `size` / `len` / `length` method-surface rows.
- Do not prune receiver-surface rows.
- Do not remove enum/fallback skeletons in this card.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_size_min.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The MIR-call route-policy `size` / `len` / `length` method-surface mirror rows
are gone, and the no-growth baseline dropped from `classifiers=17 rows=17` to
`classifiers=14 rows=14`.
