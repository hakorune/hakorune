# COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-REFRESH-001

Date: 2026-06-19
Status: accepted
Scope: compiler Recipe/CorePlan taskization

## Decision

Keep the rust-subset app-front lane and compiler Recipe/CorePlan acceptance
lane separate.

`read_next_number_literal()` still represents a real compiler-side shape family:
multi-stage scanner loops with internal `break` and staged state updates. The
small captured canary is already green through existing `LoopSimpleWhile` plus
flowbox/adopt break handling, so implementation must not reopen from that green
canary alone.

The next compiler task is to capture the **full failing shape**, or prove that
the current compiler already accepts it. Recursive Recipe/CorePlan remains the
right long-term model, but each new acceptance row still needs a minimal failing
fixture.

## Task Split

```text
COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
  capture read_next_number_literal full multi-stage loop/break shape
  reduce to a minimal failing fixture if it fails
  do not implement Recipe code in the intake row

COREPLAN-RECURSIVE-RECIPE-MINIMAL-FAILING-FIXTURE-SELECTION-001
  select the first compiler-only failing fixture from the queue
  prove planner_required failure before BoxCount work

COREPLAN-RECURSIVE-RECIPE-ACCEPTANCE-001
  add exactly one Recipe/CorePlan acceptance shape
  only after a failing fixture exists

COREPLAN-RECURSIVE-RECIPE-SHAPE-AUDIT-002
  periodically refresh unsupported compiler shapes found during app-front work
```

## Queue

```text
compiler_recipe_queue:
  read_next_number_literal_full_shape
  continue_inside_staged_loop
  nested_break_continue
  loop_carried_phi_scanner_shape
  return_break_continue_interaction
  multi_exit_scanner_loop

rust_subset_app_front_queue:
  else_if
  returnless_void_body
  vec_method_calls
  match_unsupported_handoff
  trait_generic_unsupported_handoff
```

## Ordering

For the current app-front lane, continue with:

```text
RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
```

For compiler acceptance, only switch lanes when the user explicitly chooses
compiler Recipe/CorePlan work or app-front work exposes a blocking failing
fixture.

## Stop Lines

```text
do not implement read_next_number_literal by-name branches
do not treat a green canary as proof that the full shape is accepted
do not mix rust-subset source-shape transport with compiler Recipe acceptance
do not add a new Recipe without a minimal failing fixture
do not use .hako source workaround to avoid compiler acceptance
```

## Report

```text
output_contract=coreplan-recursive-recipe-unsupported-shape-taskboard-refresh-v0
read_next_number_literal_full_shape_taskized=1
current_small_canary_green=1
minimal_failing_fixture_required=1
implementation_allowed=0
compiler_recipe_queue_refreshed=1
rust_subset_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
summary=ok
```
