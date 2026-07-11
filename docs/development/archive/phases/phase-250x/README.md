# Phase 250x — concurrency runtime hygiene cleanup

Status: LANDED
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

## Landed
- closed explicit/root scopes now immediately cancel late-registered futures with the latched reason
- `FutureBox` success is now single-assignment just like failed/cancelled terminals
- `TaskGroupInner` is now the shared registration/cancellation helper used by both `TaskGroupBox` and `global_hooks`
- plugin/runtime timeout wording is explicitly separated from MIR `Await` semantics in the docs

## Still out

- full owner unification between `TaskGroupBox` and `global_hooks`
- `joinAll()` / scope-exit return shape
- aggregate sibling-failure reporting
