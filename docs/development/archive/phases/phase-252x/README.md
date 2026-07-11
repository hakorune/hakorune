# Phase 252x — scope-exit first-failure surface

Status: LANDED
Date: 2026-04-13
Scope: explicit `task_scope` scope-exit only

## Goal

- stop silently dropping explicit-scope child failure on scope exit
- keep the cut explicit-scope-only
- keep `joinAll()` and aggregate failure reporting out of this slice

## Landed

- explicit `task_scope` exit now surfaces the popped scope's latched `first_failure`
- current order is `cancel pending -> bounded join -> surface first failure`
- `joinAll(timeout_ms)` stays unchanged and still returns `void`
- implicit root scope still does not participate in this surface

## Still out

- `joinAll()` failure surface
- aggregate / multi-failure reporting
- detached / shutdown policy widening
