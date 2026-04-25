---
Status: Landed
Date: 2026-04-26
Scope: Prune the remaining generic_method.len box-name route fallbacks after current boundary behavior proved metadata-absent direct len no longer reaches .inc lowering.
Related:
  - docs/development/current/main/phases/phase-291x/291x-242-len-route-metadata-consumer-card.md
  - docs/development/current/main/phases/phase-291x/291x-245-len-route-prune-review-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_len_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-262 Len Route Bname Fallback Prune Card

## Goal

Remove the remaining `classify_generic_method_len_route` box-name fallbacks:

```text
MapBox   -> nyash.map.entry_count_i64
ArrayBox -> nyash.array.slot_len_h
```

Length lowering must now be selected by validated `generic_method.len`
CoreMethod metadata (`MapLen`, `ArrayLen`, `StringLen`) or fail before backend
lowering.

## Probe

Current default boundary behavior was rechecked before pruning.

Metadata-absent direct `MapBox.size()`:

```text
map_size_empty_no_metadata.mir.json rc=1 object=no
unsupported pure shape for current backend recipe
```

Metadata-absent direct `ArrayBox.length()`:

```text
array_length_empty_no_metadata.mir.json rc=1 object=no
unsupported pure shape for current backend recipe
```

These inputs no longer reach `.inc` route fallback. The old defensive rows were
not the accepting boundary.

## Change

`hako_llvmc_ffi_generic_method_len_policy.inc` now routes len only from the
`GenericMethodEmitPlan` metadata flags:

```text
runtime_map_size
runtime_array_len / runtime_array_string
runtime_string
```

The corresponding no-growth allowlist rows were removed.

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_size_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_size_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_length_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
```

Result after this card:

```text
core-method-contract-inc-no-growth-guard: ok classifiers=10 rows=10
```

## Non-Blocking Observation

`tools/smokes/v2/profiles/archive/core/legacy-emit-object-evidence/s3_link_run_llvmcapi_map_set_size_canary_vm.sh`
failed with `[vm/method/stub:extern_invoke]` before it could exercise len
lowering. That archive failure is hostbridge environment related and was not
used as acceptance for this cleanup.

