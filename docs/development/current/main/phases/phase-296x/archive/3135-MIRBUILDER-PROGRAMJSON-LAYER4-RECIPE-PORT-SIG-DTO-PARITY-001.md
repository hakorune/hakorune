# 3135 - MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-PARITY-001

Status: landed

## Scope

Add `ProgramJsonRecipePortSigDtoSnapshotV1` as the next Layer4 ProgramJSON
Recipe DTO capability after the expanded Exit retire-candidate checkpoint.

This owner consumes `ProgramJsonV0PhaseStateBox.parse/2`, reads `recipe_root`,
passes it through `RecipeVerifierBox.verify/2`, and snapshots the count-only
`RecipePortSigBox` output.  It does not execute RecipeMatcher, select routes,
lower MIR, mutate MIR, or allocate IDs.

## Changed

```text
RecipeVerifierBox._verify_seq:
  traverse runtime arrays with array_get until null

RecipeVerifierBox._apply_names:
  traverse runtime arrays with array_get until null

ProgramJsonRecipePortSigDtoSnapshotV1:
  ProgramJSON -> recipe_root -> RecipeVerifier -> PortSig snapshot summary
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_recipe_port_sig_dto_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonRecipePortSigDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
recipe_verifier_used=1
recipe_port_sig_snapshot_used=1
mir_json_route_green=1
runtime_parity_green=1
runtime_route_switch=0
full_recipe_matcher_execution=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
source_selfhost_claim=0
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
MIRBUILDER-PROGRAMJSON-LAYER4-RECIPE-PORT-SIG-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
