# Phase 249x — sibling failure runtime wiring

Status: LANDED
Date: 2026-04-13
Scope: explicit `task_scope` runtime cut only

## Goal

- wire the already-pinned sibling-failure policy into the current runtime scaffold
- keep implicit root scope out of sibling-failure cancellation
- keep aggregate/scope-exit surface work for the next cut

## Landed

- futures owned by explicit `task_scope` now bind to the current `TaskGroupInner`
- the first `set_failed(...)` inside that explicit scope latches the first failure
- pending siblings in that same explicit scope are cancelled as `Cancelled: sibling-failed`
- `await` and `env.future.await` surface that sibling cancellation through the existing `TaskCancelled` / `ResultBox::Err` path

## Proof

- `boxes::task_group_box::tests`
- `await_surfaces_sibling_failed_cancellation_as_task_cancelled`
- `runtime::plugin_loader_v2::enabled::extern_functions::tests::test_future_await_sibling_failed_returns_result_err`

## Still out

- aggregate failure reporting
- `joinAll()` / scope-exit return shape
- implicit root-scope sibling cancellation
