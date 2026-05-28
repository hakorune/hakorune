---
Status: Current
Date: 2026-05-28
Scope: select the expression materialization sub-owner after local SSA position probing.
Blocker: EXPRESSION-MATERIALIZATION-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-159-LOCAL-SSA-COPY-BLOCK-POSITION-PROBE.md
  - tools/allocator/mir_expression_materialization_owner_selection.py
---

# 296x-160 Expression Materialization Owner Selection

## Purpose

Narrow row159's `expression_materialization` owner into a concrete MIR shape
before opening an optimization row.

This row does not optimize.

## Required Output

```text
output_contract=hako-mimalloc-expression-materialization-owner-selection-v0
input_contract=hako-mimalloc-local-ssa-copy-position-probe-v0
selected_owner
owner_confidence
optimization_open=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-expression-materialization-owner-selection-v0
input_contract=hako-mimalloc-local-ssa-copy-position-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
expression_materialization_copy_count=29
selected_owner=field_get_result_chain
owner_confidence=medium
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
field_get_result_chain_copy_count=28
phi_result_chain_copy_count=1
top_block_owner_0=block_552:field_get_result_chain
top_block_owner_0_copy_count=14
sample_0_owner=field_get_result_chain
sample_0_block=block_552
sample_0_inst_index=4
summary=ok
```

Interpretation:

```text
The next optimization owner is not generic local SSA cleanup. It should be a
narrow MIR-builder field_get result-chain cleanup, because 28 of 29 expression
materialization copies are field_get result chains.
```

## Next

```text
row161:
  field_get_result_chain_cleanup_selection

Goal:
  select a narrow MIR builder owner for avoiding redundant copies after
  field_get where the value is immediately copied through local temporaries.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_expression_materialization_owner_selection_guard.sh
```
