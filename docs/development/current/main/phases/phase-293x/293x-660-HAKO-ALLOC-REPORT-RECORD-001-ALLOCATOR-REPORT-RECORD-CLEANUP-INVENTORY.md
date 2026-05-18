# 293x-660 HAKO-ALLOC-REPORT-RECORD-001 Allocator Report Record Cleanup Inventory

Status: selected current
Date: 2026-05-18

## Decision

Inventory allocator proof report shapes before introducing record-based report
cleanup.

This row should find one small report/record pilot candidate, or select a
focused compiler row if the current record support is not enough.

## Scope

- Inventory wide allocator proof reports and helper signatures in
  `lang/src/hako_alloc/memory/`.
- Classify each candidate as:
  - safe for current record semantics,
  - blocked by record literal / construction / read support,
  - not worth changing because the scalar fields are clearer.
- Pick at most one source pilot if current compiler support can preserve the
  existing proof output.
- If a compiler gap is found, select one focused compiler acceptance row instead
  of working around it in `.hako`.

## Stop Lines

- No allocator behavior implementation.
- No broad report rewrite.
- No backend lowering or `.inc` matcher.
- No packed/backend record lowering.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
