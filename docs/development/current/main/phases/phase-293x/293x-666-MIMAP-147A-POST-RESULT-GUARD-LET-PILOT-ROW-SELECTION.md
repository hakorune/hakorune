# 293x-666 MIMAP-147A Post Result Guard-Let Pilot Row Selection

Status: landed
Date: 2026-05-18

## Decision

Select `HAKO-ALLOC-RESULT-API-003` as the next single row.

The first pilot proved that local `Result<i64, i64>` values plus guard-let can
stay inside one same-module pure-first body without opening cross-function
`Result` direct ABI. The next row should use that already-opened local aggregate
support to normalize the two remaining `integrateLocalFree` sub-boundary checks:

```text
apply-plan record accepted?
page-apply accepted?
```

This keeps the work inside the same owner and same proof app, avoids a compiler
sidecar, and does not resume broad mimalloc behavior yet.

## Scope

- Review HAKO-ALLOC-RESULT-API-002 evidence.
- Decide whether the next row should:
  - apply Result/guard-let to one more allocator boundary,
  - add a compiler sidecar for a specific missing Result shape, or
  - resume the next ordinary mimalloc behavior/proof row.
- Keep this row docs/planning only unless it selects a concrete follow-up card.

## Stop Lines

- No source rewrite in this row.
- No broad Result API migration.
- No cross-function `Result` direct ABI support unless selected as a separate
  compiler row.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.

## Closeout

Selected next row:

```text
HAKO-ALLOC-RESULT-API-003
  allocator local-free remaining Result guard-let boundaries
```

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
