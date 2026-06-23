# 296x-1657: Same-Module Fusion Plan SSOT

Status: Complete
Date: 2026-06-24
Token: SAME-MODULE-FUSION-PLAN-SSOT-001

## Decision

Move same-module fusion window discovery out of C shim emit files.

Same-module lowering may emit selected fusion helpers, but the choice of
which instruction window is fused must come from a named plan row. Emit code
should not discover get/binop/set windows by scanning neighboring MIR
instructions.

## Scope

```text
current files:
  lang/c-abi/shims/hako_llvmc_ffi_same_module_prepass.inc
  lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc

expected shape:
  upstream plan rows list selected fusion sites
  plan rows own skipped instruction ids, helper symbol, slots, and guards
  C shim validates and emits listed rows
```

## Non-Claims

```text
new boxed payload class = 0
new canonical MIR instruction = 0
new route descriptor = 0
new object storage plan = 0
runtime fallback = 0
```

## First Step

```text
inventory exact same-module fusion windows still discovered from neighboring
instructions

classify each as:
  fusion_window_discovery
  verification_only
  emitted_helper_body

select one window family for plan-row ownership
```

## Inventory Result

```text
fusion_window_discovery:
  typed_field_rmw:
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
    current_discovery=get field -> single-use Add binop -> set same field
    current_emit=nyash.object.exact_slot_rmw_add_u64_hiii
    selected_first=1

  result_capsule_reset_batch:
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
    current_discovery=ordered four-field reset batch
    current_emit=nyash.object.exact_slot_set4_i64_hiiiii
    selected_first=0

verification_or_registry_only:
  same_module_function_plan:
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_function_plan.inc
    role=definition registry and direct-call closure, not fusion window owner
    next=P2 SAME-MODULE-DEFINITION-EDGE-PLAN-001

emitted_helper_body:
  record_success_helper:
    role=selected helper body emission
    not_in_scope=1
```

## Worker Inventory Confirmation

```text
drain_now:
  same_module_function_register_direct_use_count
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
    issue=semantic use-count discovery lives in C

  same_module_function_match_typed_field_rmw_fusion_plan_at
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
    issue=discovers field_get -> Add -> field_set window from neighboring MIR

  same_module_function_match_typed_field_rmw_fusion_plans
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
    issue=mutates skip_inst from discovered windows

  same_module_function_name_is_selected_facade_get_set_fusion_target
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_body_emit.inc
    issue=function-name allowlist selects typed-field RMW eligibility

park_this_card:
  same_module_function_match_result_capsule_reset_batch_plan
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
    reason=second same-module window family

  record-success helper bodies
    reason=helper body emission, not selected-window discovery

drain_soon_after_1657:
  boxed_sum variant_binding publication / reconstruction
    files=boxed_sum_emit, generic_lowering_prescan, same_module_prepass
    reason=boxed-sum site facts should be mandatory rows, not C state repair

  same_module_function_publish_sum_handle_if_box
    file=lang/c-abi/shims/hako_llvmc_ffi_same_module_value_metadata.inc
    reason=__hako_sum_ prefix inference is boxed-sum semantic debt
```

## Selected Slice

```text
name:
  same_module_typed_field_rmw_fusion_plan

current C-owned decision:
  field_get exact u64 slot
  direct use count of get dst is exactly 1
  later Add binop consumes get dst and delta
  later field_set writes binop dst back to same box/field
  selected helper is nyash.object.exact_slot_rmw_add_u64_hiii

new owner:
  MIR finalize / lowering-plan producer emits one plan row per selected site.

consumer:
  same-module C shim validates the row and emits the helper.
```

## Plan Row Shape

```text
same_module_fusion_plans[] row:
  kind=same_module_typed_field_rmw_add_u64
  function
  block
  get_instruction_index
  binop_instruction_index
  set_instruction_index
  skip_instruction_indices
  get_dst
  binop_dst
  box_reg
  field
  slot
  delta_reg
  helper_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
  storage=exact_slot_u64
  direct_use_count=1
```

## Implementation Tasks

```text
1. Add Rust-side plan row emission.
   - derive the same get/binop/set window before JSON emission or during
     MIR finalize / lowering-plan construction
   - row owns skip_instruction_indices and helper_symbol
   - no C shim behavior change yet

2. Add C row reader and validation.
   - read same_module_fusion_plans from function metadata
   - validate block/index/slot/storage/helper fields
   - fail closed when the row references missing instructions
   - do not rediscover direct-use counts or neighboring get/binop/set windows

3. Switch selected emission to row consumption.
   - remove get/binop/set rediscovery for typed_field_rmw
   - remove function-name allowlist as the selected typed_field_rmw owner
   - keep helper body emission unchanged
   - skip listed get/binop instructions using the row

4. Park second window family.
   - result_capsule_reset_batch remains C-discovered until a separate card
   - do not mix it into this first row migration
```

## Stop Point

Stop for design consultation instead of patching C locally if any of these are
needed:

```text
new canonical MIR instruction
new typed-object storage plan
new route descriptor kind
function-name allowlist as the only selection owner
same-module C shim still needs to rediscover neighboring ops after row exists
```

## Acceptance

```text
one concrete same-module fusion window is selected from a named plan row
same-module C shim does not rediscover that selected window from neighboring ops
same-module helper emission behavior stays green
metadata_context_region_parent_backend=green
rust_mirbuilder_converter_matrix_guard=green
runtime_try_hako_then_rust_fallback=0
```

## Closeout Evidence

```text
rust_plan_row_owner:
  src/mir/same_module_fusion_plan.rs

json_surface:
  function.metadata.same_module_fusion_plans

c_consumer:
  same_module_function_read_typed_field_rmw_fusion_plans

removed_from_active_path:
  same_module_function_register_direct_use_count
  same_module_function_match_typed_field_rmw_fusion_plan_at
  same_module_function_match_typed_field_rmw_fusion_plans
  same_module_function_is_selected_typed_field_rmw_target
  same_module_function_name_is_selected_facade_get_set_fusion_target

verification:
  cargo test -q same_module_fusion_plan
  cargo check -q
  bash tools/checks/rust_lifecycle_metadata_context_region_parent_derived_artifact_guard.sh
  bash tools/checks/rust_mirbuilder_converter_matrix_guard.sh
  bash tools/checks/current_state_pointer_guard.sh
```
