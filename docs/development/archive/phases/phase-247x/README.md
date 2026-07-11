# Phase 247x — detached / root-scope policy pin

Status: LANDED
Date: 2026-04-13
Scope: concurrency surface policy only

## Goal

- stop reading bare `nowait` as detached work
- pin the current implicit root-scope fallback to the runtime implementation that already exists
- keep detached semantics as a later explicit surface instead of letting current fallback behavior ossify by accident

## Landed

- bare `nowait` is **not** detached
- `nowait` inside explicit `task_scope` belongs to that scope
- `nowait` outside explicit `task_scope` falls back to the implicit root scope in `global_hooks`
- `env.task.cancelCurrent` cancels the active explicit scope if present, otherwise the implicit root scope
- detached remains future explicit syntax/policy; current root-scope fallback does not define detached-task shutdown or inheritance semantics

## Proof

- docs/reference/concurrency/semantics.md
- docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md
- src/runtime/global_hooks.rs
- src/boxes/task_group_box.rs
