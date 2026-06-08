---
Status: Done
Date: 2026-06-08
Scope: MIM-PORT-FMEM-130.
Related:
  - docs/development/current/main/design/fastmem-verified-direct-default-retirement-ssot.md
  - src/mir/builder/fastmem/branch.rs
  - src/mir/builder/fastmem/ops.rs
  - tools/hako_check/fastmem_capability_inventory_common.py
  - tools/hako_check/fastmem_check.py
---

# 296x-629 MIM-PORT-FMEM-130 Branch Route Condition Fact Retirement

## Purpose

Retire the dedicated FastMemory branch lowering path by recording the branch
condition as a fact and delegating the actual CFG shape to the ordinary
`if` lowering path. The fastmem lane keeps the ownerEq-based branch proof
surface, but the branch route itself is no longer a bespoke lowering owner.

## Implementation

```text
branch fact surface:
  FastMemBranchConditionFact
  FastMemBranchConditionProofKind::SourceAssumeOwnerEq

fastmem branch lowering:
  records the branch condition fact
  verifies the branch condition comes from a region-local OwnerEq MemOp
  wraps then/else bodies as ordinary programs
  delegates to the ordinary if lowering path

inventory/check:
  fastmem_branch_condition_required_owner_eq_count
  fastmem_branch_condition_owner_eq_miss_count
  fastmem_dedicated_branch_lowering_count stays closed
```

The branch route now uses the ordinary if CFG machinery, while the fastmem
side-table keeps the proof that the condition is ownerEq-backed.

## Closed

```text
dedicated fastmem branch lowering
branch route drift into remote drain vocabulary
branch condition without ownerEq evidence
silent branch fallback
```

## Verification

```bash
cargo test fastmem_source --lib
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
FastMemory branch conditions are now fact-backed and the branch CFG itself
is owned by the ordinary if lowering path.
```

## Closeout

```text
next: MIRBUILDER-FMEM-008 proposal pending
```
