---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-133.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - src/mir/builder/fastmem.rs
  - src/mir/builder/stmts/variable_stmt.rs
  - src/mir/builder/stmts/print_stmt.rs
  - src/mir/builder/stmts/return_stmt.rs
  - src/mir/builder/builder_build.rs
  - src/mir/builder/fastmem/branch.rs
---

# 296x-632 MIM-PORT-FMEM-133 Shared Statement Shell Landing

## Purpose

Record that the post-007 FastMemory statement shell now routes the ordinary-safe
statement mechanics through shared builder helpers while keeping fastmem
expression lowering and verified-direct obligations intact.

## Implementation

```text
shared shells:
  local -> shared local declaration shell over evaluated values
  print -> shared print emission shell over evaluated values
  return -> shared return optimization/emission shell over evaluated values
  variable assignment -> shared assignment update shell over evaluated values

branch gate:
  copy chains from a local binding to ownerEq are accepted as branch evidence
  ownerEq remains the required proof source
```

The transition stays observational for the fastmem region contract itself.
No allocator activation, remote-head behavior, or new accepted source shape is
opened by this slice.

## Historical Blockers

```text
shared statement shell not yet extracted
return still owned entirely by fastmem dedicated stmt logic
local default-null path still missing
branch gate rejected copy-bound ownerEq locals
```

## Verification

```bash
cargo test -q fastmem_source --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed

```text
FastMemory now shares the statement-shell mechanics for local, print, return,
and variable assignment while still keeping fastmem expression lowering and
verified-direct route facts in place.
```

## Closeout

```text
next: MIRBUILDER-FMEM-010 field route retirement
```
