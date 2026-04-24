---
Status: Landed
Date: 2026-04-25
Scope: Prune generic-method emit-kind `len`/`length`/`size` method-name mirrors after len boundary metadata fixtures landed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-206-len-boundary-metadata-fixtures-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-207 Len Emit-Kind Mirror Prune Card

## Goal

Remove the generic-method emit-kind length alias mirror:

```c
if (mname && (!strcmp(mname, "len") || !strcmp(mname, "length") || !strcmp(mname, "size"))) {
  return HAKO_LLVMC_GENERIC_METHOD_EMIT_LEN;
}
```

Length emit selection should come from MIR-owned `ArrayLen` / `MapLen` /
`StringLen` CoreMethod metadata.

## Boundary

- Remove only the emit-kind `len` / `length` / `size` method mirror.
- Do not prune MIR-call route-policy length aliases in this card.
- Do not change length lowering or helper symbols.

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

The legacy `len` / `length` / `size` emit-kind mirror rows are gone, and the
no-growth baseline dropped from `classifiers=20 rows=20` to
`classifiers=17 rows=17`. MIR-call route-policy length alias rows remain for a
separate prune card.
