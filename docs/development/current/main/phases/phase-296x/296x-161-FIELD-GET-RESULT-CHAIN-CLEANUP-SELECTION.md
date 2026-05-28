---
Status: Current
Date: 2026-05-28
Scope: select the narrow MIR builder owner for field_get result-chain cleanup.
Blocker: FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-160-EXPRESSION-MATERIALIZATION-OWNER-SELECTION.md
  - tools/allocator/mir_field_get_result_chain_cleanup_selection.py
---

# 296x-161 Field Get Result Chain Cleanup Selection

## Purpose

Select the narrow implementation owner for row160's `field_get_result_chain`
evidence before editing compiler code.

This row does not optimize.

## Required Output

```text
output_contract=hako-mimalloc-field-get-result-chain-cleanup-selection-v0
input_contract=hako-mimalloc-expression-materialization-owner-selection-v0
selected_mir_owner
selected_file
selected_function
next_row
optimization_open=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-field-get-result-chain-cleanup-selection-v0
input_contract=hako-mimalloc-expression-materialization-owner-selection-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
expression_materialization_copy_count=29
field_get_result_chain_copy_count=28
selected_expression_owner=field_get_result_chain
selected_mir_owner=mir_builder_field_access_pin_to_slot_cleanup
selected_file=src/mir/builder/fields.rs
selected_function=MirBuilder::build_field_access
rejected_owner=PlanLowerer::emit_effect(CoreEffectPlan::FieldGet)
rejected_reason=core_effect_field_get_already_emits_selected_dst_directly
owner_confidence=medium
next_row=field_get_result_chain_cleanup_implementation
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The optimization should start at `MirBuilder::build_field_access`, where AST
field access emits `FieldGet` and then pins the result with `pin_to_slot("@field")`.
The CorePlan lowerer path is rejected for this row because
`CoreEffectPlan::FieldGet` already emits directly into the selected destination.
```

## Next

```text
row162:
  field_get_result_chain_cleanup_implementation

Acceptance:
  - reduce field_get_result_chain_copy_count
  - preserve exact proof output
  - use row157 attribution diff before exact-EXE timing
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_field_get_result_chain_cleanup_selection_guard.sh
```
