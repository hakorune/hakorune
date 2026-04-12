Status: SSOT
Scope: pin the current VM-side `await` contract for Phase-0 futures.
Related:
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `docs/reference/concurrency/semantics.md`
- `docs/development/current/main/design/exception-cleanup-async.md`
- `src/backend/mir_interpreter/handlers/mod.rs`
- `src/boxes/future/mod.rs`
- `src/runner/reference/vm_hako/tests/subset_async_ref.rs`

# 244x VM Await Contract Pin

## Decision

- current VM `await` is a narrow Future-only operation
- missing `dst` / `future` fails at the subset/schema gate
- non-`Future` operands fail at runtime with a type error
- current VM `await` has no timeout result shape and no cancellation result shape

## Fixed Reading

### Shape gate

- `await` requires `dst`
- `await` requires `future`
- malformed shapes fail-fast before runtime

### Runtime gate

- `Await` reads the `future` register
- if the operand is a `Future`, it blocks until ready and returns the stored value
- if the operand is not a `Future`, it fails as `TypeError("Await expects Future in \`future\` operand")`

### Out of scope

- `task_scope.cancelAll()` interrupting `await`
- timeout-bearing `await`
- detached-task cancellation policy
- post-Phase-0 async state-machine lowering
