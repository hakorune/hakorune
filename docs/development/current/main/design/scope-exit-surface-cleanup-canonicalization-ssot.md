---
Status: SSOT
Decision: accepted C′ single-surface cleanup target
Date: 2026-08-05
Scope: scope-exit surface naming, cleanup/fini boundary, and phased parser/lowering rollout.
Related:
  - docs/reference/language/scope-exit-semantics.md
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/design/fini-cleanup-execution-contract-ssot.md
  - docs/development/current/main/design/language-result-propagation-and-exit-transaction-ssot.md
  - docs/development/current/main/design/box-lifecycle-cprime-terminal-home-finalization-ssot.md
  - docs/reference/concurrency/lock_scoped_worker_local.md
  - docs/reference/concurrency/semantics.md
---

# Scope-Exit Surface Cleanup Canonicalization SSOT

## Decision

The sole canonical public registration for a lexical/block exit action is a
standalone `cleanup { ... }` statement.

Object lifecycle uses Box-member `fini { ... }` as a non-callable terminal
Home hook. `close()`/`shutdown()` and similar names are ordinary methods, not
language syntax.

```text
cleanup:
  lexical/block exit timing
  always-run handler
  LIFO where multiple registrations apply

fini { ... }:
  Box-member terminal Home hook
  invoked only by the C′ terminal DropPlan
  no direct receiver call and no Result channel

close()/shutdown():
  ordinary domain method
  explicit timing and optional Result
```

The target internal model is:

```text
standalone cleanup
  -> CleanupRegistrationV1
  -> VerifiedExitTransactionV1

terminal Home release
  -> lifecycle DropPlan
  -> Box fini hook, reverse field release, structural drop
```

The current `FiniReg -> Try(finally)` bridge remains transitional evidence
until the implementation row replaces it. It is not target authority.

## Rationale

The old surface overloaded `fini` and multiplied cleanup spellings:

- `fini { ... }` / `local ... fini { ... }`: scope-exit handler.
- `box.fini()`: direct object finalizer call.
- standalone, local, and postfix `cleanup`: overlapping registrations.

This is mechanically workable, but it is harder to teach and easier to
misread. The canonical naming rule is now:

```text
cleanup says when a lexical action runs.
fini says what a Box does at terminal Home release.
close says that ordinary code requests a domain transition now.
```

Example target surface:

```hako
local transaction = beginTransaction()?

cleanup {
  transaction.rollbackUnlessCommitted()
}

box File {
  close(): Result<void, IoError> {
    ...
  }

  fini {
    me.closeBestEffort()
  }
}
```

## Compatibility

Existing DropScope/handler shapes are migration input only until their parser,
bridge, and source callers retire:

```hako
fini { ... }
local x = expr fini { ... }
local x = expr cleanup { ... }
expr cleanup { ... }
body catch { ... } cleanup { ... }
```

Compatibility rules:

- Do not change parser/runtime behavior in this docs-only Decision.
- Do not silently rewrite direct `obj.fini()` to `cleanup` or `close()`;
  migration classifies intent first.
- Canonical scope cleanup count is one: standalone `cleanup { ... }`.
- Scope-position `fini { ... }` retires before Box-member `fini { ... }`
  activates, so context never has two live meanings.
- Historical migration transport must not enter canonical AST/MIR/runtime
  through fallback or profile retry.

## Handler Restrictions

Cleanup handlers are not control-flow owners.

Canonical restrictions:

- `return` is forbidden inside cleanup handlers.
- `break` is forbidden inside cleanup handlers.
- `continue` is forbidden inside cleanup handlers.
- `?` is forbidden because cleanup has no recoverable outward result channel.
- `await`, `yield`, and suspension are forbidden.
- `throw` is reserved/rejected by the current surface and must not become a
  cleanup escape hatch.

Diagnostics should be stable and direct:

```text
[scope/cleanup/control-flow]
cleanup handler cannot return.
Move the return outside the cleanup block.

[scope/cleanup/control-flow]
cleanup handler cannot break or continue.
Move loop control outside the cleanup block.
```

If a legacy parser/backend still accepts a non-local exit inside `cleanup`, that
is an implementation gap. It is not canonical semantics.

## Local State Model Boundary

The four local-state concepts stay separate:

```text
local:
  lexical binding

lock<T>:
  shared mutable state

scoped:
  dynamic context for trace/request/config

worker_local:
  performance-only worker/TLS cache
```

Do not use `cleanup` to blur these boundaries:

- `lock` scopes must not cross `await` / `nowait` / `yield`.
- cleanup/finalizer handlers must not acquire locks.
- `scoped` is context only, not resource ownership.
- `worker_local` is performance only, not correctness state.

## task_scope Ordering

`task_scope` remains the structured-concurrency boundary.

Canonical ordering for explicit `task_scope` exit is:

```text
1. protect the pending value/Outcome once
2. child failure/cancel handling owned by the task scope
3. bounded join for owned children
4. lexical cleanup handlers for the exiting scope
5. release non-forwarded local Homes in reverse declaration order
6. if a release is terminal, enter the C′ lifecycle DropPlan; its Box hook
   runs before reverse field release
7. failure/cancellation or the protected pending outcome is published once
```

The task scope owns child futures. `VerifiedExitTransactionV1` owns lexical
cleanup and local Home release ordering. The lifecycle DropPlan owns terminal
Box hook and field/native teardown. Neither owner re-infers the other's plan.

## Implementation Order

This card is docs-only. Implementation belongs to the accepted family:

```text
LANGUAGE-RESULT-EXIT-C-PRIME0-I0
  cleanup-specific AST/registration product
  VerifiedExitTransactionV1
  typed Result propagation consumer
  exact backend capability/fail-fast

LANGUAGE-RESULT-EXIT-C-PRIME0-R0
  TryCatch/CatchClause cleanup encoding = 0
  local/postfix cleanup sugar = 0
  scope fini alias = 0
  ambient handler gates and fallback = 0

LANGUAGE-RESULT-EXIT-C-PRIME0-DOC0
  mandatory implementation-backed reference/grammar/parser closeout
```

Box finalization implementation is a separate Home family. The shared exit
contract connects them only through a verified Home release request.
