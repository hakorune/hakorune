Status: Done
Date: 2026-06-17
Scope: owner selection after `kilo_micro_len_substring_views` AOT coverage was
repaired.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1027-LEN-SUBSTRING-VIEWS-AOT-FAILURE-INVENTORY-001.md
Artifacts:
  - target/len-substring-views-post-coverage-owner-1028/aot_asm.log
  - target/len-substring-views-aot-failure-1027/lanes.after.log

# LEN-SUBSTRING-VIEWS-POST-COVERAGE-OWNER-SELECTION-001

## Purpose

Select the next owner after the target front became AOT-buildable.

This is not a new benchmark-specific route. The front now exposes a generic
string lowering issue:

```text
substring result handles are created once before the loop
left.length() and right.length() are called inside the loop
the backend does not retain known length facts for substring_hii results
```

## Evidence

`bench_micro_aot_asm` after the coverage fix:

```text
top_symbol=nyash.string.len_fast_h
top_symbol_percent=99.15
aot_status=ok
```

`ny_main` hot loop:

```text
call nyash.string.substring_hii      # left, before loop
call nyash.string.substring_hii      # right, before loop
loop:
  call nyash.string.len_fast_h       # left.length()
  call nyash.string.len_fast_h       # right.length()
```

Lane evidence:

```text
c_kernel_instr=1501308
c_kernel_cycles=302121
ny_kernel_instr=73806308
ny_kernel_cycles=30512946
ratio_kernel_instr=0.02
ratio_kernel_cycles=0.01
```

## Decision

```text
output_contract=len-substring-views-post-coverage-owner-selection-v0
target_front=kilo_micro_len_substring_views
selected_owner=substring_result_known_length_fact
selected_owner_confidence=high
top_symbol=nyash.string.len_fast_h
top_symbol_percent=99.15

helper_internal_optimization_selected=0
substring_hii_semantics_change=0
product_stringbox_changed=0
benchmark_name_branch=0

next_task=STRING-SUBSTRING-RESULT-LEN-FACT-001
summary=ok
```

The next seam is the generic substring backend policy: when
`nyash.string.substring_hii(handle,start,end)` is emitted and `start/end` are
known constants, the result handle should be recorded with known length
`end-start`. Existing `StringLen` lowering can then consume the existing
known-length corridor instead of calling `len_fast_h` in the loop.

## Stop Lines

```text
do not optimize nyash.string.len_fast_h internals in this row
do not specialize by benchmark name or source variable name
do not change substring_hii runtime semantics
do not remove substring materialization in this row
```

