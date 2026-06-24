# 296x-847 MIMALLOC-LEAF-ARRAY-STRING-LEN-READY-PATH-IMPLEMENTATION-001

Status: Landed
Date: 2026-06-16

## Purpose

Record the narrow ready-path implementation attempt selected by 296x-846.

The attempted code change added `with_array_text_session_ready` and routed only
`array_string_len_by_index` through the existing `with_array_box_ready` seam.
The change was rejected and reverted in the same row because the target cycles
and wall time regressed.

## Attempted Scope

```text
implementation_scope=array_string_len_readonly_ready_path
changed_file_0=crates/nyash_kernel/src/plugin/array_handle_cache.rs
changed_file_1=crates/nyash_kernel/src/plugin/array_string_slot_indexof.rs
indexof_changed=0
store_write_changed=0
arraybox_storage_changed=0
backend_route_changed=0
mirbuilder_changed=0
product_default_changed=0
```

## Measurement

Before, from 296x-844:

```text
before_ny_aot_instr=92925832
before_ny_aot_cycles=32183346
before_ny_aot_ms=10
before_ratio_instr=0.16
before_ratio_cycles=0.10
before_ratio_ms=0.40
```

Attempt:

```text
after_ny_aot_instr=89325806
after_ny_aot_cycles=54242794
after_ny_aot_ms=13
after_ratio_instr=0.16
after_ratio_cycles=0.06
after_ratio_ms=0.31
```

The instruction count moved down, but the primary target regressed in cycles and
wall time. That is not a keeper for this lane.

## Result

```text
output_contract=hako-mimalloc-leaf-array-string-len-ready-path-implementation-v0
source_evidence=296x-846,ready-path-attempt-2026-06-16
row_kind=implementation_nonkeeper
target_front=kilo_leaf_array_string_len

implementation_attempted=1
implementation_kept=0
implementation_reverted=1
keeper_claim=0
nonkeeper_reason=cycles_and_wall_time_regressed

before_ny_aot_instr=92925832
before_ny_aot_cycles=32183346
before_ny_aot_ms=10
after_ny_aot_instr=89325806
after_ny_aot_cycles=54242794
after_ny_aot_ms=13

arraybox_storage_changed=0
backend_route_changed=0
mirbuilder_changed=0
product_default_changed=0
helper_name_inference_enabled=0

selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-NEXT-OWNER-SELECTION-001
summary=ok
```

## Stop Line

```text
do not reapply with_array_text_session_ready without a new owner row
do not keep an instruction-only win when cycles and wall time regress
do not broaden this into indexOf or store paths
do not patch MIRBuilder or backend route selection from this nonkeeper
do not claim ready-path keeper for array string len
```
