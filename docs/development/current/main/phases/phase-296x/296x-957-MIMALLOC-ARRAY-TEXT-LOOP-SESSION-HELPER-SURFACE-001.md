# 296x-957 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-HELPER-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Add the runtime helper surface selected by 296x-956 for read-only array text
loop-session length summation.

This row adds the helper and tests it directly. It does not emit the helper
from the C ABI backend, enable loop-session lowering, or change product
ArrayBox/StringBox semantics.

## Implementation

Code changes:

```text
src/boxes/array/ops/text.rs
crates/nyash_kernel/src/plugin/array_handle_cache.rs
crates/nyash_kernel/src/plugin/array_string_slot.rs
crates/nyash_kernel/src/plugin/array_string_slot_indexof.rs
crates/nyash_kernel/src/plugin/array_runtime_aliases.rs
crates/nyash_kernel/src/plugin/tests.rs
```

New helper:

```text
symbol=nyash.array.string_len_sum_region_hiii
signature=i64 (i64 handle, i64 loop_bound, i64 row_modulus, i64 initial_accumulator)
```

The helper delegates to `ArrayBox::slot_text_len_sum_region_raw`, which reads
text-slot lengths under a read lock and returns:

```text
initial_accumulator + sum(length(array[step % row_modulus]) for step in 0..loop_bound)
```

The helper does not mutate slots, does not call the edit-region substrate, and
does not materialize public StringBox values.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-helper-surface-v0
selected_helper_symbol=nyash.array.string_len_sum_region_hiii
runtime_helper_enabled=1
helper_effect=readonly
helper_mutates_array=0
helper_materializes_public_stringbox=0
backend_lowering_enabled=0
backend_helper_declaration_enabled=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-GUARD-SURFACE-002
summary=ok
```

## Stop Line

```text
do not emit helper calls in this row
do not add backend declarations in this row
do not reuse mutating edit-region helpers
do not skip the post-loop Array.length addition
do not claim performance before backend lowering and measurement
```

## Proof Bundle

```bash
cargo test -p nyash_kernel array_string_len_sum_region_reads_text_slots -- --nocapture
bash tools/build_hako_llvmc_ffi.sh
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
