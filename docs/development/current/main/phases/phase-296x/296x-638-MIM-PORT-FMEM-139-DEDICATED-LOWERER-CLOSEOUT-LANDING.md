---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-139.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - src/mir/builder.rs
  - src/mir/builder/builder_init.rs
  - src/mir/builder/calls/build.rs
  - src/mir/builder/fastmem.rs
  - src/mir/builder/fastmem/branch.rs
  - src/mir/builder/fastmem/calls.rs
  - src/mir/builder/fastmem/ops.rs
  - src/mir/builder/fields.rs
  - src/mir/builder/indexing.rs
  - src/mir/builder/scope_context.rs
  - src/mir/builder/stmts/mod.rs
---

# 296x-638 MIM-PORT-FMEM-139 Dedicated Lowerer Closeout Landing

## Purpose

Record that the transitional FastMemory source lowerer has been reduced to a
thin region-entry and obligation shell. The shared builder paths now own the
field, index, numeric, and branch-condition handling that remained after the
post-007 debt inventory.

## Implementation

```text
fastmem region entry:
  register region metadata
  push fastmem region context
  delegate body lowering to the shared block/statement/expression paths
  pop fastmem region context

shared builder paths:
  field and index routes inherit the current fastmem region context
  mem.* intrinsic calls route through the small registry
  if lowering records branch-condition facts from the shared if-form path
  verified-direct evidence stays visible in inventory/check output
```

## Verification

```bash
cargo test -q fastmem_source --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The transitional fastmem source lowerer is now a thin region-entry and
obligation shell, while the shared builder paths own the remaining fastmem
field, index, numeric, and branch-condition handling.
```

## Closeout

```text
next: phase-296x next lane selection pending
```
