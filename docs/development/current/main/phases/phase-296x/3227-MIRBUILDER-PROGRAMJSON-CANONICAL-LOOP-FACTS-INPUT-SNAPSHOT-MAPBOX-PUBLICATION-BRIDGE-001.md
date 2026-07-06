# 3227 - MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-INPUT-SNAPSHOT-MAPBOX-PUBLICATION-BRIDGE-001

Status: active

## Scope

Implement the selected AOT boundary from 3226:

```text
ProgramJsonCanonicalLoopFactsInputSnapshotV1
```

The public boundary is:

```text
ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot(program_json): MapBox
```

The snapshot is a read-only MapBox/result-map publication bridge for the
covered CanonicalLoopFacts input rows. It proves that ProgramJSON
verified-recipe input can publish a runtime-readable matcher-input snapshot.

This does not execute RecipeMatcher, select routes, lower MIR, mutate MIR,
allocate IDs, switch runtime routes, or claim Source Selfhost.

Rows:

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
```

## Acceptance

```text
must call ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1
must return DirectAbi/map_handle for build_snapshot/1
must verify through ProgramJsonV0PhaseStateBox.parse + RecipeVerifierBox.verify
must publish ok=1, source=verified_recipe, matcher_input_present=1
must publish VarLtInt loop condition and AddVarInt update tokens for both rows
must keep recipe_matcher_executed=0
must keep route_selection/lowering/mutation/id_allocation/runtime_switch=0
```

## Implementation Notes

The snapshot stores canonical string-like facts as small numeric token codes at
the MapBox publication boundary. `snapshot_summary/1` converts those codes back
to stable display tokens for parity output. This avoids AOT string-handle drift
when reading MapBox fields, while keeping the public boundary structured.

The Rust route planner now has a narrow ProgramJSON read-only map snapshot
publication contract for:

```text
ProgramJsonRecipeBodiesRuntimePublicationBridgeBox.build_publication/1
ProgramJsonCanonicalLoopFactsInputSnapshotBox.build_snapshot/1
```

This is not a generic `MapBox` publication allowance.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_canonical_loop_facts_input_snapshot_mapbox_publication_bridge_gate.sh
```

Expected result:

```text
canonical_loop_facts_input_snapshot_publication_bridge=1
read_only_canonical_loop_facts_input_snapshot=1
directabi_map_handle_publication=1
recipe_matcher_executed=0
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
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-MINIMAL-001
```
