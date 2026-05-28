---
Status: Landed
Date: 2026-05-28
Scope: classify expression-materialization copy origins before reopening optimization.
Blocker: EXPRESSION-MATERIALIZATION-COPY-ORIGIN-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-178-LOCAL-SSA-COPY-KIND-POLICY-SELECTION.md
  - tools/allocator/hako_mimalloc_expression_materialization_copy_origin_probe.py
---

# 296x-179 Expression Materialization Copy Origin Probe

## Purpose

Classify the origin and sink of expression-materialization copies in
`HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1`. This row keeps
optimization closed and turns the selected local-SSA copy policy into a narrower
owner for the next row.

## Required Output

```text
output_contract=hako-mimalloc-expression-materialization-copy-origin-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
expression_materialization_copy_count=24
dominant_expression_origin=field_get
field_get_origin_copy_count=23
dominant_expression_sink=compare_eq
selected_origin_policy=field_get_expression_value_copy_chain
next_diagnostic=field_get_expression_copy_chain_policy_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
The next row should not broaden LocalSSA reuse. It should inspect field_get
expression value copy chains and decide whether the policy belongs in MIR
builder expression materialization, field access lowering, or a later copy
cleanup pass.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_expression_materialization_copy_origin_probe_guard.sh
```
