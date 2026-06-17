Status: Done
Date: 2026-06-17
Scope: guarded backend consumer for append/update observer len-sum regions.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1021-ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-GUARD-SURFACE-001.md
  - lang/c-abi/shims/hako_llvmc_ffi_observer_store_region_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
  - crates/nyash_kernel/src/plugin/array_runtime_aliases.rs

# ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-IMPLEMENTATION-001

## Purpose

Enable the first guarded backend consumer for append/update observer len-sum
regions.

The consumer uses only the MIR-owned len-sum reader and ready predicate. It does
not rescan raw MIR windows, does not reuse the store-count helper, and does not
change product ArrayBox/StringBox storage.

## Implementation

Runtime helper:

```text
nyash.array.string_indexof_suffix_store_len_sum_region_hiisisi
```

Signature:

```text
i64(
  i64 handle,
  i64 loop_bound,
  i64 row_modulus,
  ptr needle,
  i64 needle_len,
  ptr suffix,
  i64 suffix_len
)
```

The helper mutates only matching text cells and returns the scalar sum of the
updated lengths for matched iterations.

Backend lowering emits at the proven loop header block so the existing exit PHI
can consume the accumulator value from the same predecessor block:

```text
header:
  %acc = call ...store_len_sum_region_hiisisi(...)
  br label %exit
```

The observer/body/then/latch blocks are made unreachable only when
`array_text_observer_store_len_sum_region_ready(region)=1`.

## Result

```text
output_contract=hako-array-text-append-update-backend-lowering-implementation-v0
backend_lowering_enabled=1
runtime_helper_enabled=1
store_count_helper_reused=0
raw_mir_window_rescan_allowed=0
benchmark_name_branch=0
helper_name_inference=0
product_default_changed=0
winner_claim=0
summary=ok
```

## Proof Bundle

```bash
cargo test -q -p nyash_kernel array_string_indexof_suffix_store_len_sum_region_updates_and_sums_hits
bash tools/build_hako_llvmc_ffi.sh
cargo check -q --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
ARRAY-TEXT-APPEND-UPDATE-BACKEND-LOWERING-VALIDATION-001
```

Validate generated IR/reachability on the target front before any measurement
or winner claim.
