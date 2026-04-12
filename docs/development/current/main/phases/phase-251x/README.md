# Phase 251x — scope-exit structured shutdown

Status: LANDED
Date: 2026-04-13
Scope: explicit `task_scope` exit only

## Goal

- make `pop_task_scope()` close the popped explicit scope itself
- keep nested scope ownership lexical
- keep failure surface narrow: no aggregate and no scope-exit rethrow yet

## Landed

- pinned explicit scope-exit rule as `cancel -> bounded join` per popped explicit scope
- nested explicit scopes now clean up when they exit instead of waiting for the outermost scope
- lexical token ownership is restored for nested explicit scopes
- `TaskGroupInner` now owns the shared bounded-join helper used by both `TaskGroupBox` and `global_hooks`

## Still out

- `joinAll()` / scope-exit result surface beyond `void`
- aggregate sibling-failure reporting
- full detached/runtime owner redesign
