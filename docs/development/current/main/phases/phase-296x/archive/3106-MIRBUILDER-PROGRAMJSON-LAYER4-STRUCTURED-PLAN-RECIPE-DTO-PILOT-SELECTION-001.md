# 3106 - MIRBUILDER-PROGRAMJSON-LAYER4-STRUCTURED-PLAN-RECIPE-DTO-PILOT-SELECTION-001

Status: landed

## Scope

Select the first layer-4 ProgramJSON structured Plan/Recipe DTO pilot after
the covered layer-1 ProgramJSON traversal retire-candidates.

Layer 4 means `ProgramJSON shape/facts -> structured Recipe DTO`. It does not
mean MIR mutation, backend lowering, ID allocation, route selection, full
RecipeMatcher execution, or runtime route switch.

## Selected Pilot

```text
ProgramJsonLoopRecipeDtoPilotV1
```

Target existing `.hako` route:

```text
ProgramJsonV0PhaseStateBox
  -> ProgramJsonV0PhaseStateConsumerBox
  -> LoopStmtHandler
  -> RecipeItemBox
  -> RecipeVerifierBox
```

This is selected because it already crosses the layer-4 seam: ProgramJSON is
traversed, a structured `RecipeItemBox.loop_item` DTO is constructed, and the
result is intended to pass through `RecipeVerifierBox` before any lower claim.

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-LOOP-RECIPE-DTO-PARITY-001
```

The next card must add an implementation/parity gate that proves covered
ProgramJSON loop rows produce a structured Recipe DTO and verified recipe
token. A token-snapshot-only facade is not enough.

## Non-Claims

```text
implementation_done = 0
parity_gate_green = 0
recipe_dto_migration_done = 0
runtime_route_switch = 0
rust_bootstrap_deleted = 0
rust_astnode_projector_retired = 0
full_astnode_projector_retired = 0
programjson_full_parser_claim = 0
programjson_all_shapes_supported = 0
source_selfhost_claim = 0
hako_adopted_decision = 0
full_recipe_matcher_execution = 0
route_selection_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
id_allocation_migration = 0
new_backend_route = 0
new_abi = 0
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_structured_plan_recipe_dto_pilot_selection_guard.sh
```
