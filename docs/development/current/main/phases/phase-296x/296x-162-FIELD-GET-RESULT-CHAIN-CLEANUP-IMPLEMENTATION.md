---
Status: Current
Date: 2026-05-28
Scope: implement the narrow field_get result-chain cleanup in MirBuilder::build_field_access.
Blocker: FIELD-GET-RESULT-CHAIN-CLEANUP-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-161-FIELD-GET-RESULT-CHAIN-CLEANUP-SELECTION.md
  - src/mir/builder/fields.rs
---

# 296x-162 Field Get Result Chain Cleanup Implementation

## Purpose

Apply the selected row161 implementation owner. `MirBuilder::build_field_access`
emitted `FieldGet` and then immediately pinned the field result with
`pin_to_slot("@field")`, creating redundant field-get result copy chains in
the object-lifecycle small-alloc hot path.

This row removes that unconditional field-result pinning and returns the
`FieldGet` result directly.

## Required Output

```text
output_contract=field-get-result-chain-cleanup-implementation-v0
selected_mir_owner=mir_builder_field_access_pin_to_slot_cleanup
field_get_result_chain_copy_delta
proof_summary=ok
summary=ok
```

## Evidence

Structural after/before evidence:

```text
before_instruction_count=190
after_instruction_count=180
delta_instruction_count=-10
before_copy_count=98
after_copy_count=88
delta_copy_count=-10
before_local_ssa_copy_count=48
after_local_ssa_copy_count=38
delta_local_ssa_copy_count=-10
before_expression_materialization_copy_count=29
after_expression_materialization_copy_count=24
delta_expression_materialization_copy_count=-5
before_field_get_result_chain_copy_count=28
after_field_get_result_chain_copy_count=23
delta_field_get_result_chain_copy_count=-5
```

Semantic proof:

```text
proof_app=apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
allocation_count=524288
free_count=524288
select_page_single_fast_path_count=524288
select_page_single_fallback_count=0
release_known_page_fast_path_count=524288
release_known_page_fallback_count=0
proof_summary=ok
```

Stop lines:

```text
exact_exe_timing_measurement_executed=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Next

```text
row163:
  post-field-get-result-chain-cleanup-measurement

Goal:
  run the selected exact-EXE measurement after the structural keeper.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_field_get_result_chain_cleanup_implementation_guard.sh
```
