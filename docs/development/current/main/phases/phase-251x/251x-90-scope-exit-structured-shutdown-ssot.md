Status: SSOT
Date: 2026-04-13
Scope: explicit `task_scope` exit behavior only.

# 251x Scope-Exit Structured Shutdown

## Decision

Current explicit `task_scope` exit is pinned as:

1. cancel pending child futures owned by the popped explicit scope with `scope-exit-cancelled`
2. bounded-join that same explicit scope
3. do not rethrow `first_failure`
4. do not surface aggregate failure yet

## Nested-scope reading

- each explicit scope owns its own child futures
- each explicit scope must clean up when that scope exits
- inner explicit scopes must not be deferred to the outermost scope
- current cancellation-token ownership is lexical to the active explicit scope

## Why this cut

- current `pop_task_scope()` only bounded-joined the outermost scope
- that left inner explicit scopes structurally under-defined
- this cut closes the scope-exit ownership rule without widening the user-facing error model
