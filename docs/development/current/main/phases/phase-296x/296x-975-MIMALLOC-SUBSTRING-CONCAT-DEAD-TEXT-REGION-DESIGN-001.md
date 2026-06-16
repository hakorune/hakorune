# 296x-975 MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-DESIGN-001

Status: Landed
Date: 2026-06-17

## Purpose

Design the narrow exact-AOT proof surface for the
`dead_loop_carried_text_materialization_region` owner selected in 296x-974.

This row is design-only. It does not emit a replacement loop, delete loop
bodies, add backend lowering, or change StringBox/product runtime behavior.

## Problem

`kilo_micro_substring_concat` has a closed observable result:

```text
return_value = loop_count * (base_len + inserted_len) + base_len
             = 300000 * 18 + 16
             = 5400016
```

The active AOT backend already returns that constant, but it still emits the
loop-carried local text byte materialization:

```text
left = substring(text, 0, split)
right = substring(text, split, len)
out = left + "xx" + right
text = out.substring(1, len + 1)
```

The next optimization must prove the loop text state is unpublished and
unobserved before suppressing this materialization.

## Design

Introduce a passive plan surface, not a backend rewrite:

```text
StringDeadTextRegionPlan:
  loop_header
  loop_body
  loop_exit
  text_phi_value
  text_initial_value
  loop_index_phi_value
  loop_bound_const
  base_len_const
  inserted_len_const
  accumulator_phi_value
  accumulator_initial_const
  accumulator_delta_const
  closed_return_value
  publication_boundary=none
  final_text_content_observed=0
```

This plan is backend-consumable only after separate reader and guard rows.

## Required Proofs

```text
loop_bound_const known
base_len_const known
inserted_len_const known
accumulator_delta_const = base_len_const + inserted_len_const
text length after rotation == base_len_const
final returned value uses accumulator and text.length only
final text content is not observed
text value is not stored to array/map/field
text value is not passed to plugin/extern/public Box API
text value does not cross task/future/channel boundary
substring/concat outputs are local-only
```

Unknown means no plan.

## V0 Accepted Shape

Accept only the current structural family:

```text
split = len / 2
left = text.substring(0, split)
right = text.substring(split, len)
out = left + const_text + right
acc = acc + out.length()
text = out.substring(1, len + 1)
```

The shape must be detected from MIR values and route metadata, not source text.

## Explicit Non-Goals

```text
do not add a benchmark-name branch
do not hardcode 300000, 18, 16, or 5400016 by source name
do not delete StringBox or substring helper semantics
do not rewrite product runtime StringBox storage
do not infer from helper symbol alone
do not treat arbitrary substring/concat as dead
do not fold if final text content is observed
```

## Planned Rows

```text
MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-PLAN-SURFACE-001:
  add passive MIR metadata producer and JSON export

MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-BACKEND-READER-SURFACE-001:
  add C ABI reader only

MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-GUARD-SURFACE-001:
  fix backend seam and post target

MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-IMPLEMENTATION-001:
  enable lowering only if guard row proves the plan
```

## Result

```text
output_contract=hako-mimalloc-substring-concat-dead-text-region-design-v0
row_kind=design
implementation_started=0

selected_plan_surface=StringDeadTextRegionPlan
backend_lowering_enabled=0
product_stringbox_storage_changed=0
runtime_helper_changed=0
benchmark_name_branch_allowed=0
helper_name_inference_allowed=0

selected_next=MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-PLAN-SURFACE-001
summary=ok
```
