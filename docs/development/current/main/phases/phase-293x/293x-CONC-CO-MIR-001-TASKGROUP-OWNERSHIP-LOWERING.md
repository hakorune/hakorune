# CONC-CO-MIR-001 TaskGroup Ownership Lowering

Status: Implemented
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
- `src/mir/builder/stmts/task_scope_stmt.rs`
- `src/runtime/global_hooks.rs`
- `src/runtime/context_snapshot.rs`
- `tools/smokes/v2/profiles/integration/async/co_task_scope_vm.sh`

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

## Decision

`CONC-CO-MIR-001` v0 uses runtime hook calls.

```text
co_taskgroup_lowering_shape=runtime_hook_calls
co_taskgroup_pop_error_policy=fail_fast
co_taskgroup_future_registration_owner=runtime_global_hooks
co_taskgroup_new_mir_opcode_count=0
co_early_exit_policy=normal_completion_only
program_json_co_lowering_enabled=0
llvm_co_lowering_enabled=0
```

Responsibility split:

```text
runtime::global_hooks:
  owns TaskGroup stack, future registration, context snapshot binding,
  scope-exit cancellation/join, and first-failure surfacing

MIRBuilder:
  owns lexical placement of enter/exit calls only

VM:
  dispatches the existing Call/Extern shape to runtime hooks

Program JSON / LLVM:
  stay fail-fast / unsupported in this row
```

MIR shape:

```text
Call { callee: Extern("env.task_scope.push") }
body
Call { callee: Extern("env.task_scope.pop") }
```

This row intentionally does not add `TaskScopeEnter` / `TaskScopeExit` MIR
opcodes. It proves executable ownership first.

## Alternatives Considered

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

Decision:

```text
selected for v0
pop_task_scope() Err -> fail-fast
early exit -> unsupported / fail-fast until CONC-CO-MIR-002
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

## Error And Exit Policy

`pop_task_scope()` errors are v0 fail-fast.

```text
co_taskgroup_pop_error_policy=fail_fast
co_taskgroup_pop_error_silent_ignore_count=0
```

Rationale:

```text
pop_task_scope() Err represents structured ownership failure:
  first child failure
  timeout-like scope exit failure
  runtime task-group ownership error

Ignoring it would make co/task_scope silently lose failure ownership.
```

Diagnostic tag:

```text
[freeze:contract][co/pop_task_scope_failed]
```

`CONC-CO-MIR-001` v0 is normal-completion-only.

```text
supported:
  co body lowers and reaches normal completion

unsupported in v0:
  function-level return crossing a co/task_scope boundary
  throw crossing a co/task_scope boundary
  break/continue escaping a co/task_scope boundary
```

Reason:

```text
push
body
pop
```

is only correct when `body` reaches the post-body `pop`. Early exits need
finally-like scope-exit lowering and are a later row.

Diagnostic tag:

```text
[freeze:contract][co/early-exit-unsupported]
```

Later row:

```text
CONC-CO-MIR-002:
  scope-exit cleanup lowering for early exits
  all exits route through pop
  explicit pop Err propagation policy
```

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
program_json_co_lowering_enabled=0
llvm_co_lowering_enabled=0
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
Program JSON task-scope lowering
LLVM co/task_scope lowering
early-exit cleanup lowering
```

## Task Breakdown

### CONC-CO-MIR-001A: decision pin

Status: implemented.

```text
scope=docs only
co_taskgroup_lowering_shape=runtime_hook_calls
co_taskgroup_pop_error_policy=fail_fast
co_taskgroup_future_registration_owner=runtime_global_hooks
co_taskgroup_new_mir_opcode_count=0
co_early_exit_policy=normal_completion_only
program_json_co_lowering_enabled=0
llvm_co_lowering_enabled=0
```

### CONC-CO-MIR-001B: VM extern hooks

Status: implemented.

Add VM extern dispatch for:

```text
env.task_scope.push -> runtime::global_hooks::push_task_scope()
env.task_scope.pop  -> runtime::global_hooks::pop_task_scope()
```

Acceptance:

```text
push returns Void
pop success returns Void
pop Err becomes fail-fast VM error
pop Err is not ignored
```

### CONC-CO-MIR-001C: MIRBuilder lexical lowering

Status: implemented.

Lower `co` / compatibility `task_scope` as:

```text
emit env.task_scope.push
lower body
emit env.task_scope.pop
```

Acceptance:

```text
co/task_scope lowers enter before the body
co/task_scope lowers exit after normal body completion
pop_task_scope() errors are not ignored
nowait inside explicit co/task_scope still uses FutureNew
Future registration remains owned by runtime hooks
nowait outside explicit co/task_scope keeps implicit root behavior
context snapshot propagation remains explicit-scope only
Program JSON / LLVM route widening remains closed unless separately owned
return/throw/break/continue escaping co/task_scope fail-fast in v0
```

### CONC-CO-MIR-001D: fixtures and guards

Status: implemented.

Positive fixture:

```hako
co {
    local fut = nowait { 41 + 1 }
    local v = await fut
}
```

Expected:

```text
FutureNew remains the child Future creation shape
registered child belongs to explicit TaskGroup scope
pop_task_scope executes on normal completion
```

Negative fixture:

```hako
co {
    return 1
}
```

Expected:

```text
fail-fast [freeze:contract][co/early-exit-unsupported]
```

Suggested proof commands:

```text
cargo test -q --lib runtime::global_hooks
cargo test -q --lib backend::mir_interpreter::handlers::async_contract_tests
bash tools/smokes/v2/profiles/integration/async/async_min_vm.sh
bash tools/smokes/v2/profiles/integration/async/co_task_scope_vm.sh
bash tools/checks/current_state_pointer_guard.sh
```

Implemented proof commands:

```text
cargo check -q
cargo test -q --lib runtime::global_hooks
cargo test -q --lib backend::mir_interpreter::handlers::async_contract_tests
cargo build --release -q
bash tools/smokes/v2/profiles/integration/async/async_min_vm.sh
bash tools/smokes/v2/profiles/integration/async/co_task_scope_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Report Vocabulary

```text
co_taskgroup_lowering_shape=runtime_hook_calls
co_taskgroup_push_call_count=<n>
co_taskgroup_pop_call_count=<n>
co_taskgroup_pop_error_policy=fail_fast
co_taskgroup_pop_error_silent_ignore_count=0
co_taskgroup_future_registration_owner=runtime_global_hooks
co_taskgroup_new_mir_opcode_count=0

co_early_exit_policy=normal_completion_only
co_return_inside_scope_enabled=0
co_throw_inside_scope_enabled=0
co_break_continue_escape_enabled=0

nowait_os_thread_spawn=0
worker_pool_source_route_enabled=0
worker_scope_parser_enabled=0
worker_scope_mir_lowering_enabled=0
channel_route_mir_lowering_enabled=0
sync_box_mir_lowering_enabled=0
context_snapshot_mir_lowering_enabled=0
program_json_co_lowering_enabled=0
llvm_co_lowering_enabled=0
```

## Next Step

Next implementation row:

```text
CONC-CO-MIR-002:
  scope-exit cleanup lowering for early exits
  all exits route through pop
  explicit pop Err propagation policy
```
