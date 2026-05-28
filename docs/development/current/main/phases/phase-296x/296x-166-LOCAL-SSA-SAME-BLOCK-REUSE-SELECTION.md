---
Status: Current
Date: 2026-05-28
Scope: select the LocalSSA same-block reuse implementation owner.
Blocker: LOCAL-SSA-SAME-BLOCK-REUSE-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-165-FIELD-GET-RESULT-CHAIN-FOLLOW-ON-PROBE.md
  - tools/allocator/mir_local_ssa_same_block_reuse_selection.py
---

# 296x-166 LocalSSA Same-Block Reuse Selection

## Purpose

Select the narrow compiler owner for the remaining field_get result-chain copy
surface. Row165 showed that all relevant field_get result-chain copies have
same-block field_get origins, so this row selects `LocalSSA::ensure_inner`
same-block reuse before opening an implementation patch.

## Required Output

```text
output_contract=hako-mimalloc-local-ssa-same-block-reuse-selection-v0
selected_owner=local_ssa_same_block_reuse
selected_file=src/mir/builder/ssa/local.rs
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-local-ssa-same-block-reuse-selection-v0
input_contract=hako-mimalloc-field-get-result-chain-follow-on-probe-v0
selected_owner=local_ssa_same_block_reuse
selected_file=src/mir/builder/ssa/local.rs
selected_function=ensure_inner
selected_rule=return_original_value_when_def_block_is_current_block
selected_scope=all_local_ssa_kinds_with_current_block_definition
guarded_boundary=non_dominating_and_cross_block_values_keep_existing_copy_path
rejected_owner=phi_incoming_copy_cleanup
rejected_reason=phi_incoming_is_consumer_not_origin;same_block_origin_count_equals_field_get_chain_count
rejected_owner_2=source_hako_rewrite
rejected_reason_2=remaining_surface_is_compiler_local_ssa_same_block_materialization
implementation_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Next

```text
row167:
  local_ssa_same_block_reuse_implementation

Goal:
  return the original ValueId from LocalSSA when the value is already defined in
  the current block, preserving existing copy/fail-fast behavior for cross-block
  and non-dominating values.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_local_ssa_same_block_reuse_selection_guard.sh
```
