# 296x-934 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-EXPORT-001

Status: Landed
Date: 2026-06-16

## Purpose

Export metadata-only `ArrayTextLoopSessionPlan` entries to MIR JSON so backend
rows can consume an explicit plan later instead of rediscovering loop-session
legality from raw instruction windows.

This row does not add C ABI/backend consumption or runtime loop-session
lowering.

## Implementation

```text
src/mir/array_text_loop_session_plan.rs:
  expose read-only plan proof getters

src/runner/mir_json_emit/array_metadata.rs:
  emit metadata.array_text_loop_session_plans

src/runner/mir_json_emit/tests/array_routes.rs:
  build_mir_json_root_emits_array_text_loop_session_plans
```

Export shape:

```text
route_id=array_text.loop_session.plan
loop_header=<bb>
loop_exit=<bb>
array_value=<value>
index_value=<value>
len_call_count=<n>
same_array_handle=<bool>
read_only_region=<bool>
no_mutation_region=<bool>
no_drop_or_publication_boundary=<bool>
index_domain_guarded=<bool>
backend_session_lowering_allowed=<bool>
first_reject_reason=<null|string>
mir_json_export_only=1
backend_consumer_enabled=0
```

## Evidence

The selected front now emits a plan:

```text
target_front=kilo_leaf_array_string_len
function=main
array_text_loop_session_plan_count=1
loop_header=25
loop_exit=28
array_value=5
index_value=72
backend_session_lowering_allowed=1
mir_json_export_only=1
backend_consumer_enabled=0
```

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-plan-export-v0
array_text_loop_session_plan_json_export_enabled=1
array_text_loop_session_backend_consumer_enabled=0
backend_loop_session_lowering_enabled=0
product_default_changed=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-CONSUMER-DESIGN-001
summary=ok
```

## Proof Bundle

```bash
cargo test --lib build_mir_json_root_emits_array_text_loop_session_plans -- --nocapture
cargo check --bin hakorune
cargo run --quiet --bin hakorune -- --emit-mir-json \
  /tmp/kilo_leaf_array_string_len.loop_session_export.mir.json \
  benchmarks/bench_kilo_leaf_array_string_len.hako
```

## Stop Line

```text
do not add C ABI/backend loop-session consumption in this row
do not lower loop-session runtime calls from MIR JSON export alone
do not infer backend legality from helper names
do not change product ArrayBox/StringBox behavior
```
