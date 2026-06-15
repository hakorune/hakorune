---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001
Scope: Implement the narrow `nyash_array_length_h` helper fast path selected by
  296x-707.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-707-MIMALLOC-ARRAY-LEN-HELPER-FASTPATH-PROBE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-ARRAY-LEN-HELPER-BORROWED-READY-IMPLEMENTATION-001

## Purpose

Switch only `nyash_array_length_h` to the existing borrowed read-only
`with_array_box_ready` path.

This keeps:

```text
nyash_array_length_h ABI unchanged
nyash.array.len_h alias unchanged
nyash.array.slot_len_h alias unchanged
ArrayBox.length semantics unchanged
product default NyRT unchanged
```

## Allowed Change

```text
crates/nyash_kernel/src/plugin/array_compat.rs:
  nyash_array_length_h(handle)
    from: with_array_box(handle, |arr| arr.len() as i64).unwrap_or(0)
    to:   with_array_box_ready(handle, |arr| arr.len() as i64).unwrap_or(0)
```

`with_array_box_ready` borrows under the existing host-handle registry path and
does not clone/cache the `Arc<dyn NyashBox>` for this read-only length query.

## Required Output

```text
output_contract=hako-mimalloc-array-len-helper-borrowed-ready-implementation-v0
source_evidence=296x-707
target_symbol=nyash_array_length_h
helper_abi_changed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
array_length_helper_uses_borrowed_ready=1
nyash_array_length_h_tests_green=1
body_timing_remeasured=1
winner_claim=<0|1>
summary=ok
```

## Stop Line

```text
do not change helper ABI
do not change aliases
do not change MIRBuilder or LLVM lowering
do not change tracked .hako source
do not change Arc/object substrate
do not change product defaults
```

## Acceptance

```text
output_contract=hako-mimalloc-array-len-helper-borrowed-ready-implementation-v0
source_evidence=296x-707
target_symbol=nyash_array_length_h
helper_abi_changed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
array_length_helper_uses_borrowed_ready=1
nyash_array_length_h_tests_green=1
body_timing_remeasured=1
perf_runs=10
in_process_operation_repeat=65536
body_elapsed_ns_before=54000000
body_elapsed_ns_after=53000000
top_symbol_percent_before=72.06
top_symbol_percent_after=68.13
winner_claim=1
next_task=MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001
summary=ok
```

## Result

The narrow helper switch was kept.

```text
output_contract=hako-mimalloc-array-len-helper-borrowed-ready-implementation-v0
source_evidence=296x-707
target_symbol=nyash_array_length_h
helper_abi_changed=0
product_default_changed=0
source_hako_changed=0
compiler_lowering_changed=0
runtime_object_changed=0
array_length_helper_uses_borrowed_ready=1
nyash_array_length_h_tests_green=1
body_timing_remeasured=1
perf_runs=10
in_process_operation_repeat=65536
body_elapsed_ns_before=54000000
body_elapsed_ns_after=53000000
top_symbol_percent_before=72.06
top_symbol_percent_after=68.13
winner_claim=1
next_task=MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001
summary=ok
```

Evidence:

```text
artifact_dir=/tmp/hakorune_708_array_len_impl.1781487334.report.artifacts.d
perf_report=/tmp/hakorune_708_array_len_impl.1781487334.report.artifacts.d/perf-report.txt
perf_annotate=/tmp/hakorune_708_array_len_impl.1781487334.report.artifacts.d/perf-annotate.txt
```

The result is a small keeper, not a new broad optimization direction. Remaining
samples still sit inside the handle registry / typed handle boundary, so the
next row must refresh ownership before any further implementation.
