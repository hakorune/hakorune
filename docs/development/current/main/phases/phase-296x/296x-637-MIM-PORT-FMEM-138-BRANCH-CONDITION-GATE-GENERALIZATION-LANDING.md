---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-138.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-630-MIM-PORT-FMEM-131-FASTMEM-DEDICATED-LOWERER-REMAINING-TASK-ORDER.md
  - src/mir/builder/fastmem.rs
  - src/mir/builder/fastmem/branch.rs
  - src/mir/builder/if_form.rs
  - src/mir/builder/fastmem/tests/branch.rs
  - tools/hako_check/fastmem_check.py
---

# 296x-637 MIM-PORT-FMEM-138 Branch Condition Gate Generalization Landing

## Purpose

Record that the remaining FastMemory branch condition gate now carries the
ownerEq route fact through the ordinary `if` lowering path without requiring
the fastmem branch wrapper to own AST-specific condition interpretation.

## Implementation

```text
fastmem branch lowering:
  lower the condition through the fastmem expression path
  verify ownerEq provenance from the lowered value
  record FastMemBranchConditionFact
  delegate branch shape emission to the shared if_form helper

if_form:
  accepts pre-lowered condition values for shared branch shape emission

tests:
  direct mem.ownerEq branch conditions are accepted
  ownerEq alias conditions still pass
```

## Verification

```bash
cargo test -q fastmem_source_lowers_owner_eq_branch_cfg_pilot --lib
cargo test -q fastmem_source_lowers_direct_owner_eq_branch_cfg_pilot --lib
cargo test -q fastmem_source --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The remaining FastMemory branch gate now consumes ownerEq route facts through
the shared if-form emission path, and the fastmem branch wrapper no longer
owns AST-specific condition interpretation.
```

## Closeout

```text
next: MIRBUILDER-FMEM-015 dedicated lowerer closeout
```
