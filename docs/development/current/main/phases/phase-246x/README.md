# Phase 246x: await cancelled first cut

Status: Landed

Purpose
- wire the first user-visible `Cancelled(reason)` path for scope-owned futures

Scope
- pin docs for `Cancelled(reason)` as a narrow scope-owned future contract
- add cancelled terminal state to `FutureBox`
- make `task_scope.cancelAll()` / current-scope cancellation cancel owned pending futures
- surface cancelled futures through VM `Await` and plugin/runtime `env.future.await`
- keep detached/root-scope policy out of scope

Acceptance
- live docs state that current cancellation is scope-owned-future only
- `FutureBox` can complete as cancelled
- VM `Await` surfaces cancelled futures as `TaskCancelled`
- plugin/runtime `env.future.await` surfaces cancelled futures as `ResultBox::Err`

Follow-on
- detached/root-scope policy
- deadline/timeout folded into `Cancelled(reason)`
