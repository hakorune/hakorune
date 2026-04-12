Status: SSOT
Scope: first implementation cut for `Cancelled(reason)` in pre-selfhost async.
Related:
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `src/boxes/future/mod.rs`
- `src/boxes/task_group_box.rs`
- `src/runtime/global_hooks.rs`

# 246x Await Cancelled First Cut

## Decision

- current `Cancelled(reason)` is limited to scope-owned pending futures
- the stable current reason is `scope-cancelled`
- detached/root-scope semantics remain undecided

## Fixed Reading

### Current cancellation surface

- `task_scope.cancelAll()` cancels owned pending futures
- current-scope cancellation also cancels futures registered under the active scope scaffold
- completed futures are not rewritten into cancelled futures

### Surface mapping

- VM `Await` on a cancelled future fails as `VMError::TaskCancelled(<stringified reason>)`
- plugin/runtime `env.future.await` returns `ResultBox::Err(reason)`
- the current cancellation reason payload is an `ErrorBox("Cancelled", "...")`

### Out of scope

- detached tasks
- implicit root-scope policy
- cancellation of arbitrary blocking APIs
- deadline/timeout integration
