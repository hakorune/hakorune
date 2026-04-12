# Phase 250x — concurrency runtime hygiene cleanup

Status: ACTIVE
Date: 2026-04-13
Scope: review follow-up for current task-scope/future runtime cleanliness

## Goal

- close the correctness gaps found in the worker review without widening the async surface
- keep `.hako -> MIR` lowering unchanged
- improve owner-seam cleanliness only where it directly supports current correctness

## Review Findings In Scope

- late future registration after scope cancel / sibling failure must not escape structured cancellation
- `FutureBox` success must be terminal (single-assignment)
- plugin/runtime timeout wording must be separated from MIR `Await` semantics

## Planned slices

1. late-registration immediate cancel
2. future single-assignment
3. owner-seam wording cleanup (`TaskGroupBox` / `global_hooks`) without full runtime redesign

## Still out

- full owner unification between `TaskGroupBox` and `global_hooks`
- `joinAll()` / scope-exit return shape
- aggregate sibling-failure reporting
