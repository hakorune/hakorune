# 3228 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-MINIMAL-001

Status: active

## Scope

Implement the minimal observe-only RecipeMatcher execution boundary selected
after the CanonicalLoopFacts input snapshot publication bridge:

```text
ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot(snapshot): MapBox
```

The input is the read-only `ProgramJsonCanonicalLoopFactsInputSnapshotV1`
MapBox from 3227. The output is a read-only matcher result snapshot shaped like
Rust `RecipeContractKind::LoopWithExit { has_break, has_continue, has_return }`
for the covered rows.

This does not make the ProgramJSON route the RecipeMatcher authority. It does
not select routes, lower MIR, mutate MIR, allocate IDs, switch runtime routes,
or claim Source Selfhost.

Rows:

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
```

## Acceptance

```text
must call ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1
must call ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1
must return DirectAbi/map_handle for match_snapshot/1
must consume matcher_input_present=1 readonly snapshot rows
must publish LoopWithExit with has_return=1, has_break=0, has_continue=0
must keep observe_only=1
must keep full_recipe_matcher_execution=0
must keep route_selection/lowering/mutation/id_allocation/runtime_switch=0
```

## Implementation Notes

The `.hako` owner consumes the structured MapBox snapshot rather than parsing a
string summary. It intentionally implements only the currently published
`LoopWithExit` contract projection from the snapshot fields.

The Rust route planner extends the existing narrow ProgramJSON read-only map
snapshot publication contract only for:

```text
ProgramJsonRecipeMatcherExecutionBoundaryBox.match_snapshot/1
```

This remains a scoped MapBox input/result boundary, not a generic MapBox
publication allowance and not planner integration.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_execution_boundary_minimal_gate.sh
```

Expected result:

```text
recipe_matcher_execution_boundary_minimal=1
observe_only_recipe_matcher_execution=1
recipe_matcher_input_snapshot_consumed=1
recipe_matcher_input_authority=0
full_recipe_matcher_execution=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-SHADOW-PARITY-001
```
