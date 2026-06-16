# 296x-952 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-REGION-PAYLOAD-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory the region payload required to lower the selected
`kilo_leaf_array_string_len` loop-session front after 296x-937 proved that the
current `ArrayTextLoopSessionPlan` payload is not enough for backend lowering.

This row is inventory-only. It does not extend MIR metadata, add a runtime
helper, enable C ABI lowering, or change ArrayBox/StringBox product behavior.

## Evidence

Command:

```bash
cargo run --quiet --bin hakorune -- --emit-mir-json \
  /tmp/kilo_leaf_array_string_len.region_payload.current.mir.json \
  benchmarks/bench_kilo_leaf_array_string_len.hako
```

The current exported plan proves the legality of the loop-session window:

```text
route_id=array_text.loop_session.plan
loop_header=25
loop_exit=28
array_value=5
index_value=72
len_call_count=1
same_array_handle=1
read_only_region=1
no_mutation_region=1
no_drop_or_publication_boundary=1
index_domain_guarded=1
backend_session_lowering_allowed=1
backend_consumer_enabled=0
```

The target loop requires more than legality. The backend also needs the region
mapping currently present only in raw MIR:

```text
preheader_block=21
loop_header=25
loop_body=26
loop_exit=28

array_value=5

loop_index_phi_value=52
loop_index_initial_value=51
loop_index_initial_const=0
loop_index_next_value=53
loop_bound_value=65
loop_bound_const=600000

row_index_value=72
row_modulus_value=74
row_modulus_const=64

len_value=60

accumulator_phi_value=56
accumulator_initial_value=50
accumulator_initial_const=0
accumulator_next_value=61
exit_accumulator_value=56
```

The post-loop `lines.length()` remains outside the loop-session payload:

```text
post_loop_array_length_value=87
function_return_value=92
```

That means the first loop-session lowering shape should produce the accumulated
length result only. The existing exit block may still combine that result with
the post-loop array length.

## Runtime Helper Inventory

Existing region helpers cover edit/observer shapes:

```text
nyash.array.string_insert_mid_subrange_len_store_region_hiisi
nyash.array.string_lenhalf_insert_mid_periodic_indexof_suffix_region_*
nyash.array.string_indexof_suffix_store_region_hisisi
```

No existing helper matches the read-only length-sum region shape:

```text
array handle
loop bound
row modulus
initial accumulator
return accumulated string length sum
```

## Decision

The next row should not enable backend lowering yet. It should decide the shape
of a MIR-owned region payload and whether a runtime helper contract is worth
adding for the read-only length-sum session.

```text
output_contract=hako-mimalloc-array-text-loop-session-region-payload-inventory-v0
target_front=kilo_leaf_array_string_len
row_kind=inventory

current_plan_legality_payload_complete=1
current_plan_region_payload_complete=0
region_payload_required=1
runtime_length_sum_region_helper_available=0
backend_lowering_enabled=0
backend_consumer_enabled=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-REGION-PAYLOAD-DESIGN-001
summary=ok
```

## Stop Line

```text
do not infer loop bound, row modulus, accumulator, or result placement in C
do not enable backend_consumer_enabled from the current payload
do not treat post-loop Array.length as part of the loop-session region
do not add a runtime helper without a MIR-owned payload contract
do not change ArrayBox/StringBox product runtime defaults
do not claim a Hako-vs-C winner from this inventory
```

## Proof Bundle

```bash
cargo run --quiet --bin hakorune -- --emit-mir-json \
  /tmp/kilo_leaf_array_string_len.region_payload.current.mir.json \
  benchmarks/bench_kilo_leaf_array_string_len.hako
jq '.functions[] | select(.name=="main") | .metadata.array_text_loop_session_plans' \
  /tmp/kilo_leaf_array_string_len.region_payload.current.mir.json
jq '.functions[] | select(.name=="main") | {blocks: [.blocks[] | select(.id==21 or .id==25 or .id==26 or .id==28)]}' \
  /tmp/kilo_leaf_array_string_len.region_payload.current.mir.json
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
