Status: Done
Date: 2026-06-17
Scope: guarded backend enablement surface for append/update observer len-sum regions.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1020-ARRAY-TEXT-APPEND-UPDATE-BACKEND-READER-SURFACE-001.md
  - lang/c-abi/shims/hako_llvmc_ffi_observer_store_region_metadata.inc

# ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-GUARD-SURFACE-001

## Purpose

Fix the backend lowering guard surface for the append/update observer len-sum
region now that the MIR payload and C ABI reader exist.

This row is guard-only. It does not emit a helper call, add a helper
declaration, or change product ArrayBox/StringBox behavior.

## Guard Surface

New predicate:

```text
array_text_observer_store_len_sum_region_ready(region)
```

The backend implementation row may emit only when:

```text
region.matched=1
region.begin_to_header_block == region.header_block
region.loop_bound_const >= 0
region.row_modulus_const > 0
region.length_result_value >= 0
region.accumulator_phi_value >= 0
region.accumulator_next_value >= 0
region.accumulator_next_value != region.accumulator_phi_value
region.needle_len >= 0
region.suffix_len >= 0
```

The generic lowering setup computes `active_observer_store_len_sum_region_ready`
but does not use it to emit code in this row.

## Stop Lines

```text
backend_lowering_enabled=0
runtime_helper_enabled=0
helper_declaration_added=0
raw_mir_window_rescan_allowed=0
benchmark_name_branch=0
helper_name_inference=0
wrong_store_only_helper_route=0
winner_claim=0
```

## Result

```text
output_contract=hako-array-text-append-update-backend-lowering-guard-surface-v0
len_sum_backend_reader_ready_predicate=1
backend_lowering_enabled=0
runtime_helper_enabled=0
helper_declaration_added=0
product_default_changed=0
summary=ok
```

## Proof Bundle

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-IMPLEMENTATION-001
```

Implement the guarded consumer only through the len-sum reader and ready
predicate.
