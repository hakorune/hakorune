---
Status: Landed
Date: 2026-05-28
Scope: select the local-SSA copy kind policy before another MIR optimization.
Blocker: LOCAL-SSA-COPY-KIND-POLICY-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-177-LOCAL-SSA-DYNAMIC-WEIGHT-PROBE.md
  - tools/allocator/hako_mimalloc_local_ssa_copy_kind_policy_selection.py
---

# 296x-178 Local SSA Copy Kind Policy Selection

## Purpose

Select the narrow local-SSA copy kind to investigate next. This row keeps
optimization closed and explicitly rejects retrying the recent same-block
field-get-only reuse rule that reduced static copies but regressed exact-EXE
body timing.

## Required Output

```text
output_contract=hako-mimalloc-local-ssa-copy-kind-policy-selection-v0
dominant_dynamic_owner=local_ssa_copy_materialization
dominant_local_like_position=expression_materialization
selected_copy_kind_policy=expression_materialization_copy_policy
rejected_policy=local_ssa_same_block_field_get_reuse
next_diagnostic=expression_materialization_copy_origin_probe
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
The next row should classify expression materialization copy origins before any
new lowering patch. It should not retry broad same-block LocalSSA reuse.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_local_ssa_copy_kind_policy_selection_guard.sh
```
