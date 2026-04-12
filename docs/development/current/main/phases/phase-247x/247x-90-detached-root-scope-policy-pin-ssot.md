# 247x-90 — detached / root-scope policy pin

Status: SSOT
Date: 2026-04-13

## Decision

Current concurrency surface is pinned as follows.

1. `task_scope` is the structured-concurrency surface.
2. bare `nowait` is not detached.
3. `nowait` inside explicit `task_scope` belongs to that scope and is owned by `TaskGroupBox`.
4. `nowait` outside explicit `task_scope` falls back to the implicit root scope managed by `runtime::global_hooks`.
5. `env.task.cancelCurrent` cancels the active explicit scope if one exists; otherwise it cancels the implicit root scope.
6. detached work remains a future explicit surface. Current implicit root-scope fallback must not be described as detached semantics.

## Why this cut

- current code already has a fallback registry in `register_future_to_current_group(...)`
- leaving that behavior unnamed makes docs and runtime drift
- naming it as implicit root scope keeps the current implementation explainable without overcommitting to detached semantics

## Current guarantees

- top-level future ownership exists even without explicit `task_scope`
- current cancellation can mark those root-owned pending futures as `Cancelled: scope-cancelled`
- current policy does not promise sibling-failure cancellation, lexical join, or detached shutdown handling

## Explicit non-goals

- no detached syntax
- no detached inheritance rule for `scoped`
- no shutdown-time aggregate join/failure contract
- no scheduler/executor placement policy
