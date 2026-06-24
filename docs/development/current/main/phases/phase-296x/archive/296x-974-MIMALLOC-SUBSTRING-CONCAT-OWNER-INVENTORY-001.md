# 296x-974 MIMALLOC-SUBSTRING-CONCAT-OWNER-INVENTORY-001

Status: Landed
Date: 2026-06-17

## Purpose

Inventory the selected `kilo_micro_substring_concat` front before any
implementation.

This row is observation-only. It does not patch compiler lowering, StringBox,
runtime helpers, C ABI shims, product runtime behavior, or benchmark source.

## Front

```text
target_front=kilo_micro_substring_concat
target_source=benchmarks/bench_kilo_micro_substring_concat.hako
c_pair_source=benchmarks/c/bench_kilo_micro_substring_concat.c
```

Source shape:

```text
loop 300000:
  split = len / 2
  left = text.substring(0, split)
  right = text.substring(split, len)
  out = left + "xx" + right
  acc += out.length()
  text = out.substring(1, len + 1)
return acc + text.length()
```

## Measurement Evidence

```text
c_kernel_instr=1501307
c_kernel_cycles=303776
ny_kernel_instr=4803111
ny_kernel_cycles=4806781
ratio_kernel_instr=0.31
ratio_kernel_cycles=0.06
aot_status=ok
```

Micro-ASM command:

```bash
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat ny_main 3
```

Observed `ny_main`:

```text
loop 300000:
  stack byte-copy left slice
  stack byte-copy right slice
  write "xx"
  rotate local text bytes
return 0x5265d0
```

The returned value is already closed:

```text
300000 * 18 + 16 = 5400016 = 0x5265d0
```

## MIR Evidence

MIR metadata shows direct String routes but no region owner yet:

```text
generic_method_routes:
  b0.i3  StringLen       receiver=8  result=5  demand=scalar_i64
  b19.i8 StringSubstring receiver=20 result=25 demand=read_ref
  b19.i9 StringSubstring receiver=20 result=26 demand=read_ref

range_index_facts:
  body_bb=19
  index_value=14
  lower_value=10
  upper_exclusive_value=40
  step=1
```

The loop body still contains:

```text
b19.i8  substring(text, 0, split) -> left
b19.i9  substring(text, split, len) -> right
b19     extern nyash.string.substring_concat3_hhhii(left, "xx", right, 1, len+1) -> text_next
```

## C Pair Observation

The C pair does not materialize the string bytes in the optimized hot loop. It
keeps only:

```text
loop 300000:
  acc += 18
return acc & 0xff
```

This is not a StringBox runtime-helper owner by itself. It is evidence that the
closed local text materialization is dead for the final observable result.

## Owner

```text
selected_owner_family=dead_loop_carried_text_materialization_region
selected_owner_confidence=medium
not_owner=StringBox_storage
not_owner=runtime_helper_boundary
not_owner=product_string_substring_semantics
```

The owner should be handled as an exact-AOT region proof:

```text
local text state is unpublished
only observable result is length-derived accumulator
text content after loop is not observed
return value is closed-form
```

## Stop Line

```text
do not patch substring helper internals from this evidence
do not change StringBox storage
do not infer from helper name nyash.string.substring_concat3_hhhii
do not replace by benchmark-name constant
do not remove loop body unless a metadata proof owns:
  - unpublished local text state
  - closed loop bound
  - constant length delta
  - final content unobserved
  - preserved return value
```

## Result

```text
output_contract=hako-mimalloc-substring-concat-owner-inventory-v0
row_kind=inventory
implementation_started=0

target_front=kilo_micro_substring_concat
selected_owner_family=dead_loop_carried_text_materialization_region
selected_owner_confidence=medium

substring_call_count=2
concat3_region_call_count=1
loop_bound_const=300000
per_iteration_length_delta=2
observable_accumulator_delta=18
closed_return_value=5400016

product_default_changed=0
runtime_helper_changed=0
backend_lowering_changed=0
winner_claim=0

selected_next=MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-DESIGN-001
summary=ok
```

## Proof Bundle

```bash
bash tools/perf/bench_micro_c_vs_aot_lanes.sh kilo_micro_substring_concat 1 3 100
KEEP_PERF_MICROASM_ARTIFACTS=1 PERF_MICROASM_RUNNER_MODE=direct \
  bash tools/perf/bench_micro_aot_asm.sh kilo_micro_substring_concat ny_main 3
env HAKO_STAGE1_MODE=emit-mir HAKO_EMIT_MIR_JSON=1 STAGE1_EMIT_MIR_JSON=1 \
  target/release/hakorune --emit-mir-json /tmp/kilo_micro_substring_concat.owner.mir.json \
  benchmarks/bench_kilo_micro_substring_concat.hako
```
