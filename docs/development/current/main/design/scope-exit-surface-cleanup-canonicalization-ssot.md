---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: scope-exit surface naming, cleanup/fini boundary, and phased parser/lowering rollout.
Related:
  - docs/reference/language/scope-exit-semantics.md
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/design/fini-cleanup-execution-contract-ssot.md
  - docs/reference/concurrency/lock_scoped_worker_local.md
  - docs/reference/concurrency/semantics.md
---

# Scope-Exit Surface Cleanup Canonicalization SSOT

## Decision

The canonical public concept for lexical/block exit handlers is `cleanup`.

Object lifecycle keeps `fini()` as the object finalizer method.

```text
cleanup:
  lexical/block exit timing
  always-run handler
  LIFO through DropScope where multiple handlers apply

fini():
  object lifecycle hook
  Alive -> Dead transition
  idempotent resource finalizer
```

The internal model does not change:

```text
cleanup / legacy scope fini surface
  -> DropScope handler registration
  -> finally-style execution channel

object fini()
  -> object lifecycle finalizer call
```

## Rationale

The old surface has two meanings for `fini`:

- `fini { ... }` / `local ... fini { ... }`: scope-exit handler.
- `box.fini()`: object finalizer.

This is mechanically workable, but it is harder to teach and easier to
misread. The canonical naming rule is now:

```text
cleanup says when cleanup runs.
fini() says what the object does when finalized.
```

Example target surface:

```hako
local f = open(path) cleanup {
  f.fini()
}

{
  work()
} catch e {
  log(e)
} cleanup {
  release()
}

box File {
  fini() {
    me.close()
  }
}
```

## Compatibility

Existing live DropScope syntax remains a compatibility alias until a parser and
bridge row explicitly retires or rewrites it:

```hako
fini { ... }
local x = expr fini { ... }
```

Compatibility rules:

- Do not remove `fini { ... }` in this docs-only card.
- Do not silently rewrite `box.fini()` to `cleanup`.
- New docs and examples should prefer `cleanup` terminology for scope/block
  exit handlers.
- Reference docs must call `fini { ... }` a legacy DropScope alias, not the
  canonical spelling.

## Handler Restrictions

Cleanup handlers are not control-flow owners.

Canonical restrictions:

- `return` is forbidden inside cleanup handlers.
- `break` is forbidden inside cleanup handlers.
- `continue` is forbidden inside cleanup handlers.
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
1. child failure/cancel handling owned by the task scope
2. bounded join for owned children
3. lexical cleanup handlers for the exiting scope
4. local binding drop
5. object fini() if ownership actually ends
6. failure/cancellation propagation
```

The task scope owns child futures. DropScope owns lexical cleanup. Object
ownership owns `fini()`. These owners must not be merged.

## Implementation Order

This card is docs-only. Implementation must be split:

1. parser vocabulary row for `cleanup { ... }` and `local ... cleanup { ... }`
   if those surfaces are not already live in the target frontend.
2. JSON/MIR lowering row that maps canonical cleanup syntax to the existing
   DropScope/finally channel.
3. verifier/diagnostic row for cleanup non-local-exit rejection.
4. compatibility row deciding whether `fini { ... }` remains accepted forever
   or becomes a warning/deprecated alias.

Each row needs its own fixture/gate and must not change object `fini()`
semantics.
