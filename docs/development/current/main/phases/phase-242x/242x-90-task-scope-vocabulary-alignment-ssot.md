Status: SSOT
Scope: align structured-concurrency vocabulary for the simplified thread/task design without changing Phase-0 async lowering.
Related:
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `src/boxes/task_group_box.rs`
- `src/runtime/global_hooks.rs`

# 242x Task-Scope Vocabulary Alignment

## Decision

- user-facing structured concurrency reads as `task_scope`
- current runtime scaffold behind that boundary is `TaskGroupBox`
- runtime ownership hooks are `push_task_scope()` / `pop_task_scope()`
- `RoutineScopeBox` is historical wording only and should not be used as the current name

## Fixed Reading

### In scope

- align docs and code vocabulary for structured concurrency
- describe current scope-owned lifecycle as `cancelAll()` / `joinAll(timeout_ms)` plus best-effort bounded join on scope exit
- keep `nowait` / `await` Phase-0 semantics and current MIR vocabulary unchanged

### Out of scope

- detached tasks
- first-failure-cancels-siblings policy
- aggregate failure reporting
- final `await` failure/cancel contract
- cleanup/state-machine lowering

## Current Implementation Contract

- `TaskGroupBox` owns registered child futures for the active task scope
- `cancelAll()` only marks the current group as cancelled
- `joinAll(timeout_ms)` is best-effort and bounded
- `push_task_scope()` / `pop_task_scope()` provide the current runtime scaffold for structured ownership

## Why This Cut Is Narrow

- current drift is naming drift, not runtime capability drift
- renaming boxes or widening async semantics in the same phase would mix vocabulary cleanup with behavior change
- the safe cut is to fix the SSOT and pin the current scaffold with focused tests
