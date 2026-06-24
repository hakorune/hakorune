# 296x-962 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-LENGTH-TABLE-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Implement the helper-local length table selected by 296x-961.

This row targets only the read-only region helper:

```text
ArrayBox::slot_text_len_sum_region_raw()
```

It does not change ArrayBox product storage, `nyash.array.push_hh`, StringBox,
BorrowedHandleBox, MIRBuilder, or backend lowering.

## Implementation

Before:

```text
for step in 0..loop_bound:
  idx = step % row_modulus
  len = slot[idx].as_str_fast().len()
  total += len
```

After:

```text
table_len = min(loop_bound, row_modulus)

for idx in 0..table_len:
  lengths[idx] = slot[idx].as_str_fast().len()

for step in 0..loop_bound:
  idx = step % row_modulus
  total += lengths[idx]
```

The `min(loop_bound, row_modulus)` bound preserves the previous semantics when
`loop_bound < row_modulus`: the helper must not read untouched rows that the
original loop would never access.

## Evidence

Unit tests:

```bash
cargo test -p nyash_kernel array_string_len_sum_region -- --nocapture
```

Result:

```text
array_string_len_sum_region_reads_text_slots ... ok
array_string_len_sum_region_reads_only_touched_row_domain ... ok
```

The new boundary test stores a single boxed string and runs:

```text
loop_bound=1
row_modulus=4
```

This catches regressions where the length table incorrectly reads all
`row_modulus` rows instead of only the touched row domain.

Paired microstat:

```bash
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_len 1 3
```

Result:

```text
c_instr=14526845
c_cycles=3191833
c_ms=4
ny_aot_instr=5928013
ny_aot_cycles=1395792
ny_aot_ms=4
ratio_instr=2.45
ratio_cycles=2.29
ratio_ms=1.00
c_ipc=4.55
ny_aot_ipc=4.25
aot_status=ok
```

Previous 296x-960 paired measurement:

```text
ny_aot_instr=20925175
ny_aot_cycles=4585694
ratio_instr=0.69
ratio_cycles=0.70
```

Improvement over 296x-960:

```text
ny_aot_instr_reduction_pct=71.67
ny_aot_cycles_reduction_pct=69.56
```

Direct executable check:

```text
Result: 5400064
exit=0
```

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-length-table-implementation-v0
target_front=kilo_leaf_array_string_len

length_table_enabled=1
length_table_scope=helper_local
product_array_storage_changed=0
array_push_route_changed=0
borrowed_handle_representation_changed=0
mirbuilder_changed=0
backend_lowering_changed=0

previous_ny_aot_instr=20925175
current_ny_aot_instr=5928013
ny_aot_instr_reduction_pct=71.67

previous_ny_aot_cycles=4585694
current_ny_aot_cycles=1395792
ny_aot_cycles_reduction_pct=69.56

current_ratio_instr=2.45
current_ratio_cycles=2.29
current_ratio_ms=1.00
exact_front_counter_winner=1
product_default_speedup_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-CLOSEOUT-001
summary=ok
```

## Interpretation

The selected front is now smaller than its C pair on instructions and cycles in
the paired microstat counter measurement. This is an exact-front counter keeper,
not a broad product-default performance claim.

The next row should close this array text loop-session lane and then return to
fresh front / owner selection. Continuing to shave this same helper would risk
overfitting the benchmark after the explicit owner has been removed.

## Stop Line

```text
do not keep optimizing this front without fresh owner selection
do not change ArrayBox storage policy from this row
do not change borrowed-handle representation from this row
do not use ratio_ms=1.00 as a wall-time winner claim
```

## Proof Bundle

```bash
cargo test -p nyash_kernel array_string_len_sum_region -- --nocapture
cargo fmt --check
cargo check --bin hakorune
bash tools/perf/build_perf_release.sh
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_len 1 3
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_leaf_array_string_len ny_main 3
/home/tomoaki/git/hakorune-selfhost/target/perf_ny_kilo_leaf_array_string_len.microasm.1489860.exe
```
