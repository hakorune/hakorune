# 296x-978 MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-BACKEND-LOWERING-GUARD-SURFACE-001

Status: Landed
Date: 2026-06-17

## Purpose

Fix the backend lowering guard surface for the substring-concat dead-text
region now that the MIR plan producer, JSON export, and C ABI reader exist.

This row is guard/design only. It does not emit replacement IR.

## Selected Seam

Use a closed-form return seam for the exact-AOT micro front:

```text
at loop_header:
  emit `ret i64 <final_return_value>`

skip:
  loop_body
  loop_exit
```

The selected seam is intentionally narrower than a general region helper:

```text
selected_backend_seam=loop_header_closed_form_return
selected_helper_symbol=none
runtime_helper_added=0
```

The current front has:

```text
loop_header=18
loop_body=19
loop_exit=21
text_phi_value=20
accumulator_phi_value=18
final_len_value=81
final_return_value=5400016
```

The implementation row may not rediscover this from raw MIR windows. It must
consume `StringDeadTextRegionPlanMetadata`.

## Guard Conditions

The backend implementation row may emit only when:

```text
plan.matched=1
plan.route_id=string.dead_text_region.plan
plan.publication_boundary=none
plan.final_text_content_observed=0
plan.loop_index_initial_const=0
plan.accumulator_initial_const=0
plan.loop_bound_const>=0
plan.base_len_const>0
plan.inserted_len_const>=0
plan.accumulator_delta_const=plan.base_len_const+plan.inserted_len_const
plan.final_return_value=
  plan.accumulator_initial_const+
  plan.loop_bound_const*plan.accumulator_delta_const+
  plan.base_len_const
plan.backend_consumer_enabled=0 before enabling row; implementation may consume despite JSON flag only in this guarded row
```

The exit block must be the dead-text terminal shape:

```text
exit computes final_len_value from text_phi_value
exit computes return as accumulator_phi_value + final_len_value
exit returns that sum
exit has no publication, store, plugin/extern, task/future/channel, or other observable side effect
```

Unknown means no lowering.

## Stop Line

The implementation must not:

```text
infer missing fields from raw MIR
branch by benchmark name
branch by source filename
branch by helper-name evidence
add a runtime helper
change product StringBox storage
change substring/concat helper semantics
apply to arbitrary substring/concat loops
emit if final text content is observed
emit if a publication boundary exists
```

## Result

```text
output_contract=hako-mimalloc-substring-concat-dead-text-region-backend-lowering-guard-surface-v0
row_kind=guard_design
implementation_started=0

selected_backend_seam=loop_header_closed_form_return
selected_emit_block=loop_header
selected_skip_blocks=loop_body,loop_exit
selected_helper_symbol=none
closed_form_return_enabled=0
backend_lowering_enabled=0
runtime_helper_added=0
product_stringbox_storage_changed=0
raw_mir_window_rescan_allowed=0
benchmark_name_branch_allowed=0
source_name_branch_allowed=0
helper_name_inference_allowed=0
winner_claim=0

selected_next=MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-BACKEND-LOWERING-IMPLEMENTATION-001
summary=ok
```

## Proof Bundle

```bash
rg -n "match_string_dead_text_region_plan_by_header_metadata|string_dead_text_region_has_plan" \
  lang/c-abi/shims/hako_llvmc_ffi_string_dead_text_region_metadata.inc
cargo run --bin hakorune -- --emit-mir-json \
  /tmp/kilo_micro_substring_concat.guard_probe.json \
  benchmarks/bench_kilo_micro_substring_concat.hako
jq '.functions[] | select(.name=="main") |
    .metadata.string_dead_text_region_plans[0] |
    {loop_header,loop_body,loop_exit,accumulator_phi_value,text_phi_value,
     final_len_value,final_return_value,publication_boundary,
     final_text_content_observed,backend_consumer_enabled}' \
  /tmp/kilo_micro_substring_concat.guard_probe.json
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
