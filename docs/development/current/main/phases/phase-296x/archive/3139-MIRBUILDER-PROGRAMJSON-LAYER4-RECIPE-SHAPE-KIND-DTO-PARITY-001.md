# 3139 - MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-PARITY-001

Status: landed

## Scope

Add `ProgramJsonRecipeShapeKindDtoSnapshotV1` as the next Layer4 ProgramJSON
Recipe DTO capability after the Recipe stmt-seq retire-candidate checkpoint.

This owner consumes `ProgramJsonV0PhaseStateBox.parse/2`, verifies
`recipe_root` through `RecipeVerifierBox.verify/2`, reads the stmt-only
sequence signature, and selects the canonical stmt-only `shape_kind` metadata.
It deliberately stops before route selection, RecipeMatcher execution, MIR
lowering, MIR mutation, and ID allocation.

The DTO keeps MapBox state inside the owner boundary and passes only narrow
scalar/string values to local helpers.  This avoids broadening AOT route
metadata for MapBox helper calls.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_shape_kind_dto_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonRecipeShapeKindDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_stmt_seq_scanner_used=1
shape_kind_selection=1
route_selection=0
mir_json_route_green=1
runtime_parity_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
```

Regression checks:

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_stmt_seq_dto_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_port_sig_dto_parity_gate.sh
```

## Non-Claims

```text
runtime route switch
full ASTNode projector retirement
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-SHAPE-KIND-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
