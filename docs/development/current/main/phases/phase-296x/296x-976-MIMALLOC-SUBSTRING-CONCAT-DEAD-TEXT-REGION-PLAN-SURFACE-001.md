# 296x-976 MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-PLAN-SURFACE-001

Status: Landed
Date: 2026-06-17

## Purpose

Add the passive MIR metadata producer and JSON export for the
`StringDeadTextRegionPlan` designed in 296x-975.

This row is a plan surface only. It does not add C ABI readers, backend
lowering, StringBox runtime changes, helper rewrites, or benchmark/source-name
branches.

## Implemented

```text
src/mir/string_dead_text_region_plan.rs
  StringDeadTextRegionPlan
  refresh_function_string_dead_text_region_plans()
  refresh_module_string_dead_text_region_plans()

src/mir/function/metadata.rs
  FunctionMetadata.string_dead_text_region_plans

src/mir/semantic_refresh.rs
  refresh order connects the plan producer near the string plan family

src/runner/mir_json_emit/metadata.rs
  metadata.string_dead_text_region_plans JSON export
```

## Accepted Shape

The producer accepts only the current structural family:

```text
split = len / 2
left = text.substring(0, split)
right = text.substring(split, len)
out = substring_concat3(left, const_text, right, 1, len + 1)
acc_next = acc + (len + const_text.len)
text_next = out
return acc_exit + text.length()
```

The plan is derived from MIR values and CFG shape. It does not key on benchmark
name, source path, helper name alone, or literal constants by source identity.

## Export Contract

```text
route_id=string.dead_text_region.plan
loop_bound_const=300000
base_len_const=16
inserted_text=xx
inserted_len_const=2
accumulator_delta_const=18
final_return_value=5400016
publication_boundary=none
final_text_content_observed=0
mir_json_export_only=1
backend_consumer_enabled=0
```

The numeric values above are observed for
`kilo_micro_substring_concat`; they are computed from the current MIR, not
hardcoded by benchmark/source name.

## Guards

```text
backend_lowering_enabled=0
backend_consumer_enabled=0
runtime_helper_changed=0
product_stringbox_storage_changed=0
benchmark_name_branch_count=0
source_name_branch_count=0
helper_name_only_inference_count=0
```

Unknown shape means no plan.

## Proof

```bash
cargo test --lib refresh_function_detects_substring_concat_dead_text_region -- --nocapture
cargo test --lib build_mir_json_root_emits_string_dead_text_region_plans -- --nocapture
cargo check --bin hakorune
cargo run --bin hakorune -- --emit-mir-json \
  /tmp/kilo_micro_substring_concat.dead_text_region.plan.json \
  benchmarks/bench_kilo_micro_substring_concat.hako
jq '.functions[] | select(.name=="main") |
    .metadata.string_dead_text_region_plans[0] |
    {route_id, loop_bound_const, base_len_const, inserted_text,
     inserted_len_const, accumulator_delta_const, final_return_value,
     publication_boundary, final_text_content_observed,
     backend_consumer_enabled}' \
  /tmp/kilo_micro_substring_concat.dead_text_region.plan.json
```

Observed JSON:

```json
{
  "route_id": "string.dead_text_region.plan",
  "loop_bound_const": 300000,
  "base_len_const": 16,
  "inserted_text": "xx",
  "inserted_len_const": 2,
  "accumulator_delta_const": 18,
  "final_return_value": 5400016,
  "publication_boundary": "none",
  "final_text_content_observed": false,
  "backend_consumer_enabled": false
}
```

## Result

```text
output_contract=hako-mimalloc-substring-concat-dead-text-region-plan-surface-v0
row_kind=implementation
behavior_changed=0

string_dead_text_region_plan_producer_enabled=1
string_dead_text_region_json_export_enabled=1
backend_reader_enabled=0
backend_lowering_enabled=0
product_stringbox_storage_changed=0
runtime_helper_changed=0

selected_next=MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-BACKEND-READER-SURFACE-001
summary=ok
```
