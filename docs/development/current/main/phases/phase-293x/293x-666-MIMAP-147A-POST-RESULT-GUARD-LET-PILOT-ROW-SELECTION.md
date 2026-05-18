# 293x-666 MIMAP-147A Post Result Guard-Let Pilot Row Selection

Status: selected current
Date: 2026-05-18

## Decision

Select the next single allocator/compiler row after the local-free
Result/guard-let pilot landed.

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

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
