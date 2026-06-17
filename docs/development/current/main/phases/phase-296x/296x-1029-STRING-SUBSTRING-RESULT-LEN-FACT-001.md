Status: Done
Date: 2026-06-17
Scope: exact-AOT substring result known-length fact for
`kilo_micro_len_substring_views`.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-1028-LEN-SUBSTRING-VIEWS-POST-COVERAGE-OWNER-SELECTION-001.md
Artifacts:
  - target/len-substring-views-result-len-fact-1029/ny_mir_builder.lenfact.log
  - target/len-substring-views-result-len-fact-1029/objdump.lenfact.txt
  - target/len-substring-views-result-len-fact-1029/lanes.lenfact.log
  - target/len-substring-views-result-len-fact-1029/aot_asm.lenfact.log

# STRING-SUBSTRING-RESULT-LEN-FACT-001

## Purpose

Record known string lengths for substring results when the backend emits
`nyash.string.substring_hii` with constant `start/end` operands.

This row does not optimize `len_fast_h` internals and does not change runtime
substring semantics. It only connects an existing backend fact corridor:

```text
substring_hii(result,start,end)
  -> put_known_string_len(result,end-start)
  -> existing StringLen lowering emits constants
```

## Implementation

`hako_llvmc_ffi_generic_method_substring_policy.inc` now has a small
`remember_substring_result_known_len()` helper. It is called from the generic
`substring_hii` fallback and from existing substring-derived direct routes that
already record `remember_string_substring_call()`.

## Evidence

Direct MIR rebuild:

```text
aot_status=ok
result=4800016
exit=16
```

Route trace:

```text
string_len_corridor result=hit reason=known_string_len extra=recv=17 len=8
string_len_corridor result=hit reason=known_string_len extra=recv=27 len=8
```

`ny_main` after the row:

```text
call nyash.box.from_i8_string_const
call nyash.box.from_i8_string_const
call nyash.string.substring_hii
call nyash.string.substring_hii
ret
```

There are no `nyash.string.len_fast_h` calls in `ny_main` after this row.

Lane measurement:

```text
before_ny_kernel_instr=73806308
after_ny_kernel_instr=6267
before_ny_kernel_cycles=30512946
after_ny_kernel_cycles=7129

after_c_kernel_instr=1501308
after_c_kernel_cycles=302225
after_ratio_kernel_instr=239.56
after_ratio_kernel_cycles=42.39
aot_status=ok
```

Micro-ASM after the row:

```text
top report is loader/startup dominated
ny_main has no sampled body owner
winner_claim_allowed=1
```

## Decision

```text
output_contract=string-substring-result-len-fact-v0
target_front=kilo_micro_len_substring_views
substring_result_known_len_fact_enabled=1
substring_hii_runtime_semantics_changed=0
len_fast_h_internal_optimization=0
benchmark_name_branch=0
source_variable_name_branch=0

len_fast_h_calls_in_ny_main_before=2
len_fast_h_calls_in_ny_main_after=0
ny_kernel_instr_before=73806308
ny_kernel_instr_after=6267
ny_kernel_cycles_before=30512946
ny_kernel_cycles_after=7129

keeper_claim=1
next_task=FRESH-COMPILER-OWNER-SELECTION-004
summary=ok
```

## Stop Lines

```text
do not change product StringBox semantics
do not optimize nyash.string.len_fast_h internals from this evidence
do not remove substring materialization in this row
do not add benchmark/source/helper-name branches
```
