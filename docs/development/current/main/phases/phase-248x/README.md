# Phase 248x — sibling-failure policy pin

Status: LANDED
Date: 2026-04-13
Scope: docs-only policy cut

## Goal

- pin what happens inside explicit `task_scope` when one child future fails
- keep aggregate/scope-exit surface work out of this cut
- keep implicit root scope separate from sibling-failure policy

## Landed

- explicit `task_scope` uses `first failure cancels siblings`
- the first failed child is the current main failure for that scope
- pending siblings are cancelled with reason `sibling-failed`
- already-ready siblings are not rewritten
- implicit root scope does not participate in sibling-failure cancellation in this cut
- aggregate reporting and scope-exit rethrow remain later-phase work

## Next

- runtime wiring for explicit-scope sibling cancellation
- then scope-exit / `joinAll()` surface
