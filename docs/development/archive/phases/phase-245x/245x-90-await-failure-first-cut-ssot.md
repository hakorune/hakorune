Status: SSOT
Scope: first implementation cut for `await` failure beyond contract/type errors.
Related:
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `docs/reference/concurrency/semantics.md`
- `docs/development/current/main/design/exception-cleanup-async.md`
- `src/boxes/future/mod.rs`
- `src/backend/mir_interpreter/handlers/mod.rs`
- `src/backend/mir_interpreter/handlers/externals.rs`
- `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`

# 245x Await Failure First Cut

## Decision

- the first user-visible `await` failure widening is `TaskFailed(error)`
- `Cancelled(reason)` stays reserved only
- timeout/deadline does not join the VM-side `await` contract in this cut

## Fixed Reading

### Failure taxonomy

- `ContractError`
  - malformed `await`
  - non-`Future` operand
- `TaskFailed(error)`
  - the future completed in a failed terminal state
  - VM `Await` / `env.future.await` in the MIR interpreter raise `VMError::TaskFailed(<stringified payload>)`
  - plugin/runtime `env.future.await` returns `ResultBox::Err(error)`
- `Cancelled(reason)`
  - reserved only

### Runtime shape

- `FutureBox` has three terminal meanings:
  - pending
  - ready(value)
  - failed(error)
- `set_result(...)` writes `ready(value)`
- `set_failed(...)` writes `failed(error)`

### Out of scope

- sibling-failure cancellation
- `task_scope.cancelAll()` interrupting `await`
- deadline/timeout folded into `Cancelled(reason)`
- detach/root-scope policy
