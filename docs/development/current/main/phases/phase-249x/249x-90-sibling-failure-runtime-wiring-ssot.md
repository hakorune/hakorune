Status: SSOT
Date: 2026-04-13
Scope: explicit-scope runtime wiring for sibling-failure cancellation.

# 249x Sibling Failure Runtime Wiring

## Decision

- explicit-scope sibling-failure policy is now runtime-active
- first failed child future latches the current first failure and cancels pending siblings with `sibling-failed`
- existing `await` surface for cancelled futures is reused; no new error kind is added in this cut

## Runtime shape

- `TaskGroupInner` owns:
  - registered strong futures
  - `first_failure`
  - a one-way `sibling_failure_seen` latch
- `FutureBox` can bind to an explicit sibling-failure scope owner
- `register_future_to_current_group(...)` binds futures to the active explicit scope only
- implicit root-scope fallback keeps ownership only; it does not participate in sibling-failure cancellation

## Fixed surface

- VM `await` on a sibling-cancelled future returns `VMError::TaskCancelled("Cancelled: sibling-failed")`
- plugin/runtime `env.future.await` on the same future returns `ResultBox::Err("Cancelled: sibling-failed")`

## Still out

- scope-exit rethrow of `first_failure`
- aggregate failure surface
- sibling failure across detached/root-scope work
