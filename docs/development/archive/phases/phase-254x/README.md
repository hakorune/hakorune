# Phase 254x — aggregate / multi-failure reporting

Status: LANDED
Date: 2026-04-13
Scope: explicit `TaskGroupBox` aggregate-failure reporting only

## Goal

- preserve more than the first child failure inside explicit structured scope ownership
- keep `joinAll()` and explicit scope exit pinned to first-failure surface only
- expose aggregate failure state through a separate owner-side report surface

## Landed slices

1. pin aggregate reporting as `TaskGroupBox.failureReport() -> ArrayBox`
2. keep report order as `[first_failure, additional_failures...]`
3. keep sibling cancellations out of aggregate causes
4. add focused regressions for empty/non-empty report

## Still out

- aggregate reporting for implicit root scope
- aggregate payload surfacing from `pop_task_scope()`
- dedicated timeout payload for `joinAll()` / scope exit
