Status: SSOT
Date: 2026-04-13
Scope: explicit `task_scope` sibling-failure policy only.

# 248x Sibling-Failure Policy Pin

## Decision

- current policy is `first failure cancels siblings`
- this applies only to explicit `task_scope`
- implicit root scope is out of scope for this policy cut

## Fixed reading

### Trigger

- a child future owned by the active explicit `task_scope` reaches `TaskFailed(error)`

### Immediate effect

- that failure becomes the current main failure for the scope
- pending siblings owned by the same explicit scope are cancelled as `Cancelled("sibling-failed")`
- already-ready siblings are not rewritten

### Not fixed yet

- aggregate failure reporting
- `joinAll()` return shape
- scope-exit rethrow policy
- implicit root scope sibling cancellation
- deadline/timeout folding
