Status: SSOT
Date: 2026-04-13
Scope: explicit `task_scope` scope-exit failure surfacing only.

# 252x Scope-Exit First-Failure Surface

## Decision

Current explicit `task_scope` exit is pinned as:

1. cancel pending child futures owned by the popped explicit scope with `scope-exit-cancelled`
2. bounded-join that same explicit scope
3. if that explicit scope has a latched `first_failure`, surface that first failure on the scope-exit path
4. do not widen `joinAll()` in this cut
5. do not add aggregate failure payloads in this cut

## Scope

- explicit `task_scope` only
- implicit root scope remains ownership-only
- current failure payload is still the first failure as string

## Why this cut

- sibling-failure runtime wiring already latched `first_failure`
- scope exit still dropped that failure when the caller did not `await` the failed child directly
- surfacing the popped scope's first failure closes the current silent-drop hole without inventing aggregate APIs yet
