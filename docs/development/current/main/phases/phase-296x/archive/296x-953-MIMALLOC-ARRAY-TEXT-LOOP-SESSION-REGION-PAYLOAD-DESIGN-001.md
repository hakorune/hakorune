# 296x-953 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-REGION-PAYLOAD-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Choose the MIR-owned payload shape needed before the C ABI backend can lower
the `kilo_leaf_array_string_len` read-only loop session.

This row is design-only. It does not add fields to `ArrayTextLoopSessionPlan`,
export new MIR JSON, add a runtime helper, or enable backend lowering.

## Decision

Reuse the existing region-mapping shape already proven by
`ArrayTextResidenceLoopRegionMapping` instead of inventing a second payload
vocabulary.

The selected payload fields are:

```text
array_root_value
loop_index_phi_value
loop_index_initial_value
loop_index_initial_const
loop_index_next_value
loop_bound_value
loop_bound_const
accumulator_phi_value
accumulator_initial_value
accumulator_initial_const
accumulator_next_value
exit_accumulator_value
row_index_value
row_modulus_value
row_modulus_const
```

For the selected front, these correspond to:

```text
array_root_value=5
loop_index_phi_value=52
loop_index_initial_value=51
loop_index_initial_const=0
loop_index_next_value=53
loop_bound_value=65
loop_bound_const=600000
accumulator_phi_value=56
accumulator_initial_value=50
accumulator_initial_const=0
accumulator_next_value=61
exit_accumulator_value=56
row_index_value=72
row_modulus_value=74
row_modulus_const=64
```

## Ownership

```text
payload_owner=ArrayTextLoopSessionPlan
payload_shape=ArrayTextResidenceLoopRegionMapping-compatible
producer_owner=src/mir/array_text_loop_session_plan.rs
json_export_owner=src/runner/mir_json_emit/array_metadata.rs
c_abi_reader_owner=lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc
backend_lowering_owner=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
```

`ArrayTextLoopSessionPlan` remains the owner for this read-only session because
the loop-session legality and the region payload must be consumed together.
`ArrayTextResidenceSessionRoute` remains owner for loopcarry store sessions.

## Runtime Helper Direction

Do not select or implement a helper yet. The payload surface must land first so
the helper signature can be chosen from MIR-owned fields rather than raw backend
inspection.

The likely first helper contract is a read-only length-sum region:

```text
input:
  array handle
  loop bound const
  row modulus const
  initial accumulator const/value

output:
  accumulated string length sum
```

The helper contract still needs a separate row to decide whether the initial
accumulator is always folded to zero or passed explicitly.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-region-payload-design-v0
target_front=kilo_leaf_array_string_len
row_kind=design

selected_payload_owner=ArrayTextLoopSessionPlan
selected_payload_shape=array_text_residence_loop_region_mapping_compatible
selected_region_payload_field_count=15
runtime_helper_contract_selected=0
backend_lowering_enabled=0
backend_consumer_enabled=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-REGION-PAYLOAD-SURFACE-001
summary=ok
```

## Stop Line

```text
do not add C ABI lowering in the payload surface row
do not scan raw MIR windows in C to fill missing payload
do not create a second incompatible region mapping vocabulary
do not merge this with runtime helper implementation
do not change product ArrayBox/StringBox behavior
```

## Proof Bundle

```bash
rg -n "ArrayTextResidenceLoopRegionMapping|loop_index_phi_value|accumulator_phi_value|row_modulus_const" \
  src/mir lang/c-abi/shims -g'*.rs' -g'*.inc'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
