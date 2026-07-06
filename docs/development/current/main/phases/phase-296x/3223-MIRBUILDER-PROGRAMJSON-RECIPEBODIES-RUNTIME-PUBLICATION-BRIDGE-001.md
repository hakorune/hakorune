# 3223 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-PUBLICATION-BRIDGE-001

Status: active

## Scope

Implement a read-only runtime publication bridge for verifier-accepted
ProgramJSON RecipeBodies DTO rows:

```text
RecipeBodiesPublicationSnapshotV1
```

The bridge publishes only a result-map / map-handle snapshot. It does not make
RecipeBodies runtime authority, execute RecipeMatcher, select routes, lower MIR,
mutate MIR, allocate IDs, switch runtime routes, or claim Source Selfhost.

Rows:

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
```

## Acceptance

```text
must call ProgramJsonRecipeBodiesRuntimePublicationBridgeBox.build_publication/1
must return DirectAbi/map_handle for build_publication/1
must publish ok=1, readonly=1, verified_recipe_present=1
must preserve body_count=4, def_count=1, update_count=2
must keep recipe_matcher_executed=0
must keep runtime_route_switch=0
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_runtime_publication_bridge_gate.sh
```

Expected result:

```text
runtime_recipe_bodies_publication_bridge=1
read_only_publication_snapshot=1
runtime_recipe_bodies_authority=0
full_recipe_matcher_execution=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-EXECUTION-BOUNDARY-MINIMAL-001
```
