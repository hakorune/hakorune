# 296x-1658: Same-Module Reset Batch Fusion Plan SSOT

Status: Complete
Date: 2026-06-24
Token: SAME-MODULE-RESET-BATCH-FUSION-PLAN-SSOT-001

## Decision

Move the second same-module fusion window family out of C shim discovery.

The typed-field RMW window is already selected by `same_module_fusion_plans`.
The remaining same-module window in the same emitter was the result-capsule
reset batch:

```text
last_page_id  = -1
last_block_id = -1
last_reason   = 0
last_ok       = 0
```

That ordered four-field reset is now selected by MIR-owned plan rows. C may
emit the helper body, but it must not discover the ordered field-set sequence
from neighboring instructions.

## Scope

```text
current files:
  src/mir/same_module_fusion_plan.rs
  src/runner/mir_json_emit/route_metadata.rs
  lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
  lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc

selected window:
  result_capsule_reset_batch

helper:
  nyash.object.exact_slot_set4_i64_hiiiii
```

## Non-Claims

```text
new boxed payload class = 0
new canonical MIR instruction = 0
new route descriptor = 0
new object storage plan = 0
runtime fallback = 0
record-success helper migration = 0
```

## Plan Row Shape

```text
same_module_fusion_plans[] row:
  kind=same_module_result_capsule_reset_batch_i64
  function
  block
  first_set_instruction_index
  set_instruction_indices
  skip_instruction_indices
  box_reg
  fields
  slots=[0,1,2,3]
  values=[-1,-1,0,0]
  helper_symbol=nyash.object.exact_slot_set4_i64_hiiiii
  storage=exact_slot_i64
```

## Closeout

```text
rust_plan_row_owner:
  src/mir/same_module_fusion_plan.rs

json_surface:
  function.metadata.same_module_fusion_plans

c_consumer:
  same_module_function_read_result_capsule_reset_batch_plan

removed_from_active_path:
  same_module_function_match_result_capsule_reset_batch_plan
  same_module_function_is_selected_result_capsule_reset_batch_target

verification:
  cargo test -q same_module_fusion_plan
  cargo check -q
  bash tools/checks/rust_lifecycle_metadata_context_region_parent_derived_artifact_guard.sh
  bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
  bash tools/checks/current_state_pointer_guard.sh
```

## Acceptance

```text
reset batch helper selection is owned by a named plan row
same-module C shim does not rediscover the reset field-set sequence
same-module helper emission behavior stays green
metadata_context_region_parent_backend=green
rust_mirbuilder_converter_matrix_guard=green
runtime_try_hako_then_rust_fallback=0
```
