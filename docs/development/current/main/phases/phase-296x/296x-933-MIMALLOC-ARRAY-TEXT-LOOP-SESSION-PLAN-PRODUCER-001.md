# 296x-933 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-PRODUCER-001

Status: Landed
Date: 2026-06-16

## Purpose

Connect the selected `Array.get(row) -> String.length()` window with the
MIR-owned modulo range-index fact and produce `ArrayTextLoopSessionPlan`
metadata.

This row is producer-only. It does not export the plan to MIR JSON, add C ABI
consumer code, or lower a loop-session runtime path.

## Implementation

```text
src/mir/function/metadata.rs:
  FunctionMetadata::array_text_loop_session_plans

src/mir/array_text_loop_session_plan.rs:
  refresh_function_array_text_loop_session_plans()
  refresh_module_array_text_loop_session_plans()

src/mir/semantic_refresh.rs:
  refresh order connects:
    range_index_facts
    array_string_len_window_routes
    array_text_loop_session_plans
```

The producer accepts only the narrow while-loop shape:

```text
header:
  branch -> body / exit

body:
  Array.get(index).length()
  jump -> header
```

Required proofs:

```text
same_array_handle=1
read_only_region=1
no_mutation_region=1
no_drop_or_publication_boundary=1
index_domain_guarded=1
```

`index_domain_guarded` is satisfied by a `RangeIndexFact` for the route's
`index_value` in the same body block. This includes the modulo-derived
`RangeIndexFactOriginKind::ModuloOfRangeIndex` added in 296x-932.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-plan-producer-v0
target_front=kilo_leaf_array_string_len
array_text_loop_session_plan_producer_enabled=1
array_text_loop_session_plan_metadata_field_enabled=1
array_text_loop_session_plan_json_export_enabled=0
array_text_loop_session_backend_consumer_enabled=0
backend_loop_session_lowering_enabled=0
product_default_changed=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-EXPORT-001
summary=ok
```

## Proof Bundle

```bash
cargo test --lib array_text_loop_session_plan -- --nocapture
cargo test --lib range_index_fact -- --nocapture
cargo fmt --check
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do not export ArrayTextLoopSessionPlan to MIR JSON in this row
do not add C ABI/backend loop-session lowering in this row
do not infer loop-session legality from helper names
do not widen beyond the single-body while loop shape
do not change product ArrayBox/StringBox behavior
```
