# 296x-954 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-REGION-PAYLOAD-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Add the passive MIR-owned region payload surface for the selected
`kilo_leaf_array_string_len` array text loop-session plan.

This row only exposes metadata. It does not add C ABI lowering, runtime helper
calls, product ArrayBox/StringBox behavior changes, or backend consumption.

## Implementation

Code changes:

```text
src/mir/array_text_loop_session_plan.rs
src/mir/array_text_loop_session_plan/region_payload.rs
src/runner/mir_json_emit/array_metadata.rs
src/runner/mir_json_emit/tests/array_routes.rs
```

`ArrayTextLoopSessionPlan` now carries an optional
`ArrayTextLoopSessionRegionPayload`. The payload is derived by the MIR producer
from the selected loop header/body/exit and exported to MIR JSON as
`region_payload`.

The region payload uses value-origin root values, matching the existing
`ArrayTextResidenceLoopRegionMapping` producer convention. For the selected
front, the emitted root payload is:

```text
array_root_value=5
loop_index_phi_value=52
loop_index_initial_value=51
loop_index_initial_const=0
loop_index_next_value=53
loop_bound_value=66
loop_bound_const=600000
accumulator_phi_value=56
accumulator_initial_value=50
accumulator_initial_const=0
accumulator_next_value=61
exit_accumulator_value=56
row_index_value=72
row_modulus_value=75
row_modulus_const=64
```

The raw MIR inventory row listed the copy operands
`loop_bound_value=65` and `row_modulus_value=74`. This surface intentionally
exports their roots `66` and `75` because the region-mapping family resolves
copy chains before publishing payload values.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-region-payload-surface-v0
target_front=kilo_leaf_array_string_len
row_kind=implementation_surface

region_payload_surface_enabled=1
region_payload_json_export_enabled=1
selected_region_payload_field_count=15
payload_value_policy=value_origin_root
backend_consumer_enabled=0
backend_lowering_enabled=0
runtime_helper_enabled=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-PAYLOAD-READER-SURFACE-001
summary=ok
```

## Stop Line

```text
do not add C ABI lowering in this row
do not add runtime helper calls in this row
do not infer region payload by raw MIR scanning in C
do not enable backend_consumer_enabled
do not change product ArrayBox/StringBox behavior
do not claim a Hako-vs-C winner from this metadata surface
```

## Proof Bundle

```bash
cargo test --lib array_text_loop_session_plan -- --nocapture
cargo test --lib build_mir_json_root_emits_array_text_loop_session_plans -- --nocapture
cargo run --quiet --bin hakorune -- --emit-mir-json \
  /tmp/kilo_leaf_array_string_len.region_payload.surface.mir.json \
  benchmarks/bench_kilo_leaf_array_string_len.hako
jq '.functions[] | select(.name=="main") | .metadata.array_text_loop_session_plans[0].region_payload' \
  /tmp/kilo_leaf_array_string_len.region_payload.surface.mir.json
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
