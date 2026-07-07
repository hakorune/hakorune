# 3231 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-DUAL-RUN-SHADOW-GUARD-001

Status: active

## Scope

Formalize the 3230 recommended default:

```text
A_SHADOW_ONLY_DUAL_RUN_GUARD
```

The guard runs the ProgramJSON matcher-result route beside the Rust ASTNode
authority evidence and fails on mismatch. It does not switch runtime authority.

Authority remains:

```text
Rust ASTNode route -> RecipeMatcher::try_match_loop -> PlanBuildOutcome.recipe_contract
```

Shadow remains:

```text
ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1
  -> ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1
```

## Acceptance

```text
must require 3229 RecipeMatcher shadow parity gate
must require 3230 runtime route shadow-switch design-stop guard
must compare canonical matcher-result fields
must keep runtime_authority=rust_astnode
must keep ProgramJSON route shadow-only
must fail fast on mismatch
must keep runtime_route_switch=0
must keep programjson_runtime_route_authority=0
must keep recipe_matcher_input_authority=0
must keep route_selection/lowering/mutation/id_allocation/runtime_fallback=0
```

## Implementation Notes

This is an authority-boundary guard, not a new ProgramJSON traversal capability.
It is intentionally allowed under the task-order exception for explicit
authority-boundary dual-run stop guards.

The guard reuses the existing AOT/EXE ProgramJSON path proven by 3229. It adds
the runtime-authority contract that prevents shadow parity from being mistaken
for a route switch.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_runtime_dual_run_shadow_guard.sh
```

Expected result:

```text
dual_run_shadow_guard=1
runtime_authority=rust_astnode
programjson_shadow_checked=1
dual_run_match=1
mismatch_count=0
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-EXPANDED-ROWS-001
```
