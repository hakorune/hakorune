# CONC-CO-MIR-001 TaskGroup Ownership Lowering

Status: Pending-design
Date: 2026-06-15
Scope: lower `co` / compatibility `task_scope` into explicit structured
TaskGroup ownership events after the Future MIRBuilder boundary is pinned.

Related:
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/boundary-model.md`
- `docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md`
- `docs/development/current/main/phases/phase-293x/293x-1035-CONC-FUTURE-SEM-001-MIRBUILDER-FUTURE-BOUNDARY.md`
- `src/mir/builder/exprs.rs`
- `src/mir/builder/stmts/async_stmt.rs`
- `src/runtime/global_hooks.rs`
- `src/runtime/context_snapshot.rs`

## Purpose

`CONC-FUTURE-SEM-001` pins the current Future boundary:

```text
nowait -> expression -> FutureNew
await  -> expression -> Safepoint -> Await -> Safepoint
```

This row must add only the structured ownership boundary around those Future
events:

```text
co / task_scope enter
  body
  nowait children created inside the body are registered to the explicit scope
co / task_scope exit
```

It must not reinterpret `nowait` as thread spawn, activate worker-pool routing,
or widen Channel / sync-box / context lowering.

## Existing Truth

Runtime ownership already exists in `src/runtime/global_hooks.rs`:

```text
push_task_scope()
pop_task_scope()
register_future_to_current_group(future)
```

The current reference runtime contract is:

```text
explicit scope owns registered child futures
scope exit cancels pending children with scope-exit-cancelled
scope exit bounded-joins the popped group
scope exit surfaces that group's first_failure
implicit root scope remains best-effort ownership only
```

Context snapshot inheritance is also already owned by the runtime hook path:

```text
register_future_to_current_group(future) inside explicit scope
  -> captures the current context stack snapshot

register_future_to_current_group(future) outside explicit scope
  -> no context propagation
```

Therefore `CONC-CO-MIR-001` should not create a second TaskGroup truth owner.
MIRBuilder should only materialize the boundary that calls the existing owner.

## Design Question

The next implementation must choose the MIR shape for the structured boundary.

### Option A: runtime hook calls

```text
emit call runtime::global_hooks::push_task_scope
lower body
emit call runtime::global_hooks::pop_task_scope
```

Reading:

```text
MIRBuilder owns lexical placement of enter/exit.
runtime::global_hooks owns TaskGroup state and exit semantics.
```

Benefits:

```text
reuses existing runtime truth
does not add MIR opcodes
keeps Program JSON / MIR JSON vocabulary stable
keeps LLVM unsupported/fail-fast until a backend route exists
```

Risk:

```text
needs a clean error propagation shape for pop_task_scope() failures
needs finally-like cleanup if body lowering can return/throw early
```

### Option B: metadata-only event

```text
annotate a scope region as structured-concurrency owned
leave runtime hooks to a later lowering pass
```

Reading:

```text
MIRBuilder records intent only.
another stage must insert runtime ownership calls.
```

Benefits:

```text
very small first code change
```

Risk:

```text
no executable proof of TaskGroup ownership
easy to drift from FutureNew registration
adds another delayed truth boundary
```

### Option C: dedicated MIR instructions

```text
TaskScopeEnter
TaskScopeExit
```

Reading:

```text
structured concurrency becomes first-class MIR vocabulary.
```

Benefits:

```text
clear semantic marker for VM / backend / verifier
```

Risk:

```text
widens MIR opcode vocabulary
requires MIR JSON producer/reader work
requires VM and backend policy immediately
too large for the first ownership-lowering row
```

## Recommended Decision

Use Option A for the first implementation slice.

Reason:

```text
The runtime already owns TaskGroup state and context snapshot registration.
The first compiler row should only make lexical ownership executable.
Adding MIR instructions now would force every backend surface to participate
before the ownership contract itself is proven.
```

This recommendation still requires one design decision before code:

```text
How should pop_task_scope() failure surface through MIRBuilder lowering?
```

Acceptable answers are:

```text
fail-fast freeze in this row
or
explicit Result/throw propagation if a compatible existing MIR shape exists
```

Do not silently ignore `pop_task_scope()` errors.

## Stop Lines

```text
nowait_os_thread_spawn=0
worker_pool_source_route_enabled=0
worker_scope_parser_enabled=0
worker_scope_mir_lowering_enabled=0
raw_thread_parser_enabled=0
channel_route_mir_lowering_enabled=0
sync_box_mir_lowering_enabled=0
context_snapshot_mir_lowering_enabled=0
```

Do not add:

```text
new source syntax
worker-pool activation
raw thread syntax
hidden blocking Channel calls
sync-box method-entry lowering
context scope lowering
dedicated TaskScope MIR opcodes in the first slice
```

## Acceptance

Before landing code for this row, pin:

```text
co_taskgroup_lowering_shape=runtime_hook_calls
co_taskgroup_pop_error_policy=<fail_fast_or_explicit_propagation>
co_taskgroup_future_registration_owner=runtime_global_hooks
co_taskgroup_new_mir_opcode_count=0
```

Code acceptance for the eventual implementation:

```text
co/task_scope lowers enter before the body
co/task_scope lowers exit after normal body completion
pop_task_scope() errors are not ignored
nowait inside explicit co/task_scope still uses FutureNew
Future registration remains owned by runtime hooks
nowait outside explicit co/task_scope keeps implicit root behavior
context snapshot propagation remains explicit-scope only
Program JSON / LLVM route widening remains closed unless separately owned
```

Suggested proof commands:

```text
cargo test -q --lib runtime::global_hooks
cargo test -q --lib backend::mir_interpreter::handlers::async_contract_tests
bash tools/smokes/v2/profiles/integration/async/async_min_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Next Step

Stop for design consultation before code:

```text
Question:
  Should CONC-CO-MIR-001 use runtime hook calls with fail-fast
  pop_task_scope() error handling as the first executable ownership slice?

Default recommendation:
  yes; use runtime hook calls and fail-fast pop errors for v0.
```
