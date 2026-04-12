# Phase 242x: task-scope structured-concurrency vocabulary alignment

Status: Landed

Purpose
- align the new simplified thread/task design with current repo reality without widening runtime semantics

Scope
- pin the user-facing structured-concurrency term to `task_scope`
- pin the current runtime scaffold to `TaskGroupBox` plus `push_task_scope()` / `pop_task_scope()`
- retire `RoutineScopeBox` as current wording and keep it historical only
- keep `nowait` / `await` Phase-0 lowering unchanged
- leave detached tasks, sibling-failure policy, and cleanup/cancel integration to later phases

Acceptance
- reference concurrency docs no longer use `RoutineScopeBox` as the current structured-concurrency owner
- pre-selfhost async SSOT explicitly separates task-scope naming from `nowait` / `await` lowering
- code comments and focused unit tests pin the current `TaskGroupBox` scaffold contract

Follow-on
- decide `await` failure/cancel contract and detached/root-scope policy in a later concurrency phase
