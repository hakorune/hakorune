# 3234 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001

Status: landed

## Scope

Install the runtime-adjacent ProgramJSON shadow guard boundary selected by
3233.

Boundary:

```text
after try_build_outcome(ctx)
before registry candidate selection
```

The Rust route remains runtime authority. ProgramJSON matcher evidence remains
shadow-only and is verified by lifecycle gates. This card does not add
ProgramJSON input to `LoopRouteContext`.

## Acceptance

```text
must require 3233 consultation guard
must require 3232 expanded rows gate
must call the shadow guard immediately after try_build_outcome(ctx)
must call it before registry::collect_candidates or recipe-first routing
must keep the guard read-only
must keep runtime_authority=rust_astnode
must keep programjson_runtime_route_authority=0
must keep runtime_route_switch=0
must keep recipe_matcher_input_authority=0
must keep route_selection/lowering/mutation/id_allocation/runtime_fallback=0
```

## Non-Claims

```text
ProgramJSON does not write PlanBuildOutcome.recipe_contract.
ProgramJSON does not feed route registry predicates.
ProgramJSON does not compose CorePlan.
ProgramJSON does not lower or mutate MIR.
ProgramJSON does not allocate IDs.
Rust is not used as a fallback from ProgramJSON mismatch.
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_runtime_route_adjacent_shadow_guard.sh
```

Expected result:

```text
runtime_route_adjacent_shadow_guard=1
boundary_after_try_build_outcome_before_route_candidate_selection=1
runtime_authority=rust_astnode
programjson_shadow_checked_by_lifecycle_gate=1
no_downstream_write=1
programjson_input_in_loop_route_context=0
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-AUTHORITY-SWITCH-COVERAGE-FLOOR-SELECTION-001
```
