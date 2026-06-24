---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Refresh taskization for recursive Recipe/CorePlan unsupported shapes and keep json_native stability work separate.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1256-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKIZATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1264-COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-REFRESH-001.md
  - docs/development/current/main/phases/phase-296x/296x-1269-RUST-SUBSET-AFTER-LOOP-TASKBOARD-REFRESH-001.md
  - apps/rust-subset-to-hako/STATUS.md
---

# COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-REFRESH-002

## Decision

Keep `read_next_number_literal()` as a compiler Recipe/CorePlan acceptance
candidate, but do not implement it from a method name, from an app-front source
shape, or from json_native token payload stabilization.

The current stabilization path stays split:

```text
json_native stability:
  token payload / numeric semantic conversion / object-key equality

compiler Recipe/CorePlan:
  recursive acceptance of real loop exits and multi-stage scanner control-flow

rust-subset app-front:
  Rust source-shape transport or explicit Unsupported handoff
```

A temporary return to the token payload route is allowed only as stability
recovery for json_native. It is not evidence that the compiler accepts the full
`read_next_number_literal()` shape.

## Current Interpretation

The reported shape is:

```text
read_next_number_literal():
  multi-stage scanner loop
  staged state updates
  nested or conditional break
  possible loop-carried scanner state
  possible EOF/error/value multi-exit behavior
```

Recursive Recipe/CorePlan is the right direction because this is a nested
control-flow shape, not a source-to-source rewrite. However, recursive Recipe
is not a license to implement broadly without a failing fixture.

## Acceptance Rule

Before any Recipe/CorePlan implementation:

```text
1. capture the real shape or a reduced minimal fixture
2. run planner_required / relevant fast gate
3. prove the first reject/freeze owner
4. add exactly one Recipe/CorePlan acceptance shape
5. keep the fixture and gate in the same row
```

If the fixture is already green, close the row without compiler changes and pick
the next candidate.

## Unsupported Shape Queue

### Compiler Recipe/CorePlan Queue

```text
read_next_number_literal_full_shape
  status=queued
  owner=compiler Recipe/CorePlan
  next=COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001

continue_inside_staged_loop
  status=queued_after_break_shape
  owner=compiler Recipe/CorePlan
  note=park until a break-bearing fixture proves the next gap

nested_break_continue
  status=queued
  owner=compiler Recipe/CorePlan
  note=requires minimal failing fixture, not broad recursive implementation

loop_carried_phi_scanner_shape
  status=queued
  owner=compiler Recipe/CorePlan / PHI lifecycle
  note=only open after a fixture proves loop-carried state is the first owner

return_break_continue_interaction
  status=queued
  owner=compiler Recipe/CorePlan
  note=separate from simple break/continue rows

multi_exit_scanner_loop
  status=queued
  owner=compiler Recipe/CorePlan
  note=EOF/error/value exits need explicit Recipe data if unsupported
```

### RustSubset App-Front Queue

```text
trait/generic item support
  status=unsupported_handoff_only
  owner=syn adapter / RustSubset schema selection

match semantics
  status=unsupported_handoff_only
  owner=future RustSubset schema row

for-loop semantics
  status=unsupported_handoff_only
  owner=future RustSubset schema row
  note=do not desugar to while until iterator semantics are selected
```

### json_native Hardening Queue

```text
object_key_equality_false_positive
  status=active_next
  owner=json_native object key lookup
  next=JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001

number_token_payload_route
  status=regression_guarded
  owner=json_native tokenizer / ArrayBox element origin route
  note=stability route only; not compiler Recipe evidence

numeric_semantic_conversion
  status=resolved_by_parser_source_span_integer_parse
  owner=json_native parser numeric semantics
```

## Stop Lines

```text
do not add read_next_number_literal by-name branches
do not mix json_native token payload stability with compiler Recipe acceptance
do not implement recursive Recipe without a minimal failing fixture
do not use .hako workaround source changes to avoid compiler acceptance
do not add match/for semantics while they are selected only as Unsupported handoff
do not remove JsonObjectKeyMaterializer until object-key equality is owner-selected and full converter parity is green
```

## Report

```text
output_contract=coreplan-recursive-recipe-unsupported-shape-taskboard-refresh-v2
read_next_number_literal_full_shape_taskized=1
recursive_recipe_direction_preserved=1
implementation_allowed=0
minimal_failing_fixture_required=1
token_payload_route_is_json_native_stability_only=1
nonzero_number_semantics_compiler_owner=0
compiler_recipe_queue_refreshed=1
rust_subset_unsupported_handoff_queue_refreshed=1
json_native_hardening_queue_refreshed=1
active_next_json_native_task=JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001
summary=ok
```
