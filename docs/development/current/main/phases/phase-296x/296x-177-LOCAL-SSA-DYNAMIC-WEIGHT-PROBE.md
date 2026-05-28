---
Status: Current
Date: 2026-05-28
Scope: estimate dynamic workload weight for the selected local-SSA MIR owner.
Blocker: LOCAL-SSA-DYNAMIC-WEIGHT-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-176-OBJECT-LIFECYCLE-MIR-BODY-OWNER-SELECTION.md
  - tools/allocator/hako_mimalloc_local_ssa_dynamic_weight_probe.py
---

# 296x-177 Local SSA Dynamic Weight Probe

## Purpose

Convert the selected local-SSA copy owner from static MIR counts into estimated
dynamic workload operations. This keeps optimization closed and prevents a
repeat of the prior same-block field-get local-SSA non-keeper.

## Required Output

```text
output_contract=hako-mimalloc-local-ssa-dynamic-weight-probe-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
method_invocation_count=524288
dominant_dynamic_owner=local_ssa_copy_materialization
local_ssa_copy_materialization_dynamic_ops=<positive integer>
rejected_recent_nonkeeper=local_ssa_same_block_field_get_reuse
next_diagnostic=local_ssa_copy_kind_policy_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

```text
If local-SSA remains dominant after multiplying by the object-lifecycle workload
count, the next row should select which local-SSA copy kind is eligible for a
new policy. It must not retry the same-block field-get-only rule that already
regressed.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_local_ssa_dynamic_weight_probe_guard.sh
```
