# 293x-850 HAKO-ALLOC-USIZE-FIELD-GROUP-042 Next Field-Group Selection

Status: selected current
Date: 2026-05-19

## Decision

Select the next narrow allocator stored numeric field group for exact `usize`
migration after closing the modeled local-free reuse ledger count group.

This is a planning row. It does not migrate fields by itself.

## Scope

- Inspect the remaining hako_alloc stored numeric fields.
- Select one owner-local, non-negative size/count/capacity field group.
- Record why the selected fields are safe to migrate together.
- Keep signed sentinel, signed reason/status, ids/tokens, block-span sentinels,
  pointer-shaped handles, flags, and counters that intentionally stay signed
  out of scope.

## Stop Lines

- No stored-field migration in this row.
- No broad `i64` to `usize` rewrite.
- No runtime sum materialization.
- No cross-function `Result` direct ABI.
- No new backend route or `.inc` owner-name matcher.
- No real raw pointer residence, real segment-map mutation, arena backing
  execution, atomic bitmap execution, OSVM/page-source execution, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

After selection, create `HAKO-ALLOC-USIZE-FIELD-GROUP-043` for the chosen
field-group migration.
