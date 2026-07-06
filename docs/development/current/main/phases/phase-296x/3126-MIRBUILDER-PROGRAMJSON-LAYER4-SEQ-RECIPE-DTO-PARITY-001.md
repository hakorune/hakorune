# 3126 - MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-PARITY-001

Status: landed

## Scope

Select and implement `ProgramJsonSeqRecipeDtoSnapshotV1` as the next concrete
Layer4 ProgramJSON Recipe DTO capability after the 3125 If DTO retire-candidate
checkpoint.

This card adds the `.hako` owner, fixture, and parity gate for root `Seq` Recipe
DTO snapshots.  It proves MIR JSON route readiness and AOT runtime parity for
the covered stmt-only root sequence rows.

## Why Seq Next

`Seq` is the common root DTO for the already proven Loop and If Layer4 paths.
Proving root Seq separately reduces reliance on Rust ASTNode projection for
stmt-only recipe rows and gives the next Layer4 capabilities a stable parent DTO
to inspect.

## Covered Rows

```text
return_int
return_new_box
local_return_var
empty_body_reject
```

Top-level Assignment/Print rows are intentionally deferred.  In this Layer4
route they are not yet PhaseState runtime-green and should be handled as a
separate PhaseState consumer capability instead of being claimed by Seq DTO.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_seq_recipe_dto_snapshot.hako
```

The owner consumes Program(JSON v0) through `ProgramJsonV0PhaseStateBox.parse/2`,
reads `recipe_root.items`, and emits a canonical `SeqRecipeDtoSnapshotV1`
summary.  It does not execute RecipeMatcher, select routes, lower MIR, mutate
MIR, or allocate IDs.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_seq_recipe_dto_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonSeqRecipeDtoSnapshotV1
programjson_traversal_used=1
structured_recipe_dto_constructed=1
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
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
ProgramJSON full parser
HakoAdoption for a full owner
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-SEQ-RECIPE-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
