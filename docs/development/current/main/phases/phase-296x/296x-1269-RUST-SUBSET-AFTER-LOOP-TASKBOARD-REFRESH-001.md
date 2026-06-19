# RUST-SUBSET-AFTER-LOOP-TASKBOARD-REFRESH-001

Date: 2026-06-19
Status: accepted
Scope: rust-subset-to-hako app-front taskboard / compiler Recipe backlog routing

## Decision

After accepting Rust `loop { ... }` without `break` / `continue`, keep the next
work split into two lanes:

```text
rust-subset app-front:
  source-shape transport into existing RustSubset JSON and .hako emitter shapes

compiler Recipe/CorePlan:
  semantic acceptance for real loop exits, staged scanner loops, and recursive
  control-flow shapes
```

The reported `read_next_number_literal()` shape is **not** an app-front feature.
It remains a compiler Recipe/CorePlan task:

```text
read_next_number_literal:
  multi-stage scanner loop
  staged state updates
  internal break / exit edge
  possible loop-carried state
```

The current small staged-loop canary is already green through existing
`LoopSimpleWhile` plus flowbox/adopt break handling. Therefore implementation
must not start from the method name or from the green canary. The next compiler
row must capture the full real shape or produce a minimal failing fixture first.

## Next Task List

### App-Front Lane

```text
RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-009
  choose one source transport shape
  candidates:
    match unsupported handoff
    returnless typed unit function
    for-loop unsupported handoff
  non-candidates:
    break
    continue
    multi-stage scanner exits
```

Recommended next app-front task:

```text
RUST-SUBSET-SYN-ADAPTER-MATCH-UNSUPPORTED-HANDOFF-001
  accept Rust match as an explicit Unsupported node
  preserve fail-fast / handoff behavior
  do not add match semantics
```

### Compiler Recipe/CorePlan Lane

```text
COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
  capture read_next_number_literal full shape
  reduce to a minimal failing fixture if it fails
  implementation_allowed=0

COREPLAN-RECURSIVE-RECIPE-MINIMAL-FAILING-FIXTURE-SELECTION-001
  select the first compiler-only failing fixture
  prove planner_required failure before BoxCount work

COREPLAN-RECURSIVE-RECIPE-ACCEPTANCE-001
  add exactly one Recipe/CorePlan acceptance shape
  only after a failing fixture exists
```

Compiler shape queue:

```text
read_next_number_literal_full_shape
continue_inside_staged_loop
nested_break_continue
loop_carried_phi_scanner_shape
return_break_continue_interaction
multi_exit_scanner_loop
```

### JSON Native Stability Lane

```text
JSON-NATIVE-OBJECT-KEY-EQUALITY-OWNER-SELECTION-001
  classify why full fixture key lookup false-positives without JsonObjectKeyMaterializer
  keep JsonObjectKeyMaterializer temporary until full converter parity is stable
  implementation_allowed=0
```

## Stop Lines

```text
do not implement break/continue in the rust-subset app-front lane
do not special-case read_next_number_literal by name
do not reopen compiler Recipe/CorePlan implementation without a minimal failing fixture
do not restore parser WIP that destabilizes bool-return / converter validation
do not remove JsonObjectKeyMaterializer until full smoke passes without it
```

## Report

```text
output_contract=rust-subset-after-loop-taskboard-refresh-v0
loop_without_break_completed=1
next_app_front_blocker=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-009
read_next_number_literal_is_compiler_recipe_task=1
read_next_number_literal_app_front_candidate=0
minimal_failing_fixture_required_before_recipe_implementation=1
json_native_key_bridge_retire_blocked_by_full_smoke=1
summary=ok
```
