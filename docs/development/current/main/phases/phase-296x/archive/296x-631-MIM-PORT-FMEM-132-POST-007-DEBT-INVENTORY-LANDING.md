---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-132.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - tools/hako_check/fastmem_capability_inventory_common.py
  - tools/hako_check/fastmem_capability_inventory_smoke.sh
---

# 296x-631 MIM-PORT-FMEM-132 Post-007 Debt Inventory Landing

## Purpose

Record that the post-007 FastMemory inventory slice is now visible in the
inventory report. The dedicated lowerer is still transitional, but the
remaining broad AST shapes are now counted explicitly instead of staying
implicit.

## Implementation

```text
new visible counts:
  fastmem_dedicated_local_lowering_count
  fastmem_dedicated_literal_lowering_count
  fastmem_dedicated_variable_lowering_count
  fastmem_dedicated_call_lowering_count
  fastmem_dedicated_method_call_lowering_count
  fastmem_branch_condition_gate_count

existing debt counters:
  fastmem_dedicated_assignment_lowering_count
  fastmem_dedicated_branch_lowering_count
  fastmem_branch_condition_required_owner_eq_count
  fastmem_branch_condition_owner_eq_miss_count
```

The inventory stays observational. No allocator activation, remote-head
behavior, or new accepted source shape is opened by this slice.

## Closed

```text
inventory-only debt remains hidden
branch gate visibility missing
smoke zero expectations missing
new behavior
```

## Verification

```bash
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The FastMemory inventory now exposes the remaining dedicated AST-shape debt
as explicit counts, so the next row can retire shared statement handling.
```

## Closeout

```text
next: MIRBUILDER-FMEM-009 shared statement shell
```
