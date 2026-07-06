# 3124 - MIRBUILDER-PROGRAMJSON-LAYER4-IF-RECIPE-DTO-PARITY-001

Status: landed

## Scope

Select and implement `ProgramJsonIfRecipeDtoSnapshotV1` as the next concrete
Layer4 ProgramJSON Recipe DTO capability after the 3123 loop DTO retire-candidate
checkpoint.

This card is not selection-only.  It adds the `.hako` owner, a fixture, and a
single parity gate that proves MIR JSON route readiness and AOT runtime parity
for the covered If Recipe DTO rows.

## Why If Next

`IfStmtHandler` already constructs `RecipeItemBox.if_item`, and
`RecipeVerifierBox` already accepts `If` items.  The previous loop DTO proof
made the Layer4 DTO route stable enough to move to the next structured recipe
item without reopening full RecipeMatcher execution or MIR lowering.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_if_recipe_dto_snapshot.hako
```

The owner consumes Program(JSON v0) through `ProgramJsonV0PhaseStateBox.parse/2`,
reads the resulting `recipe_root`, and emits a canonical
`IfRecipeDtoSnapshotV1` summary.

Covered shapes:

```text
local_if_then_return_else_null_return_int
local_if_then_return_else_null_return_var
local_if_then_else_assignment_return_var
if_without_local_reject
```

The snapshot owner does not return MapBox/object helpers across same-module
AOT boundaries.  The assignment-tail branch reads `Seq.items[0]` inside the
same function so object traversal remains internal to the snapshot owner rather
than widening module-generic object-return routes.

## Dynamic Token Fix

`IfStmtHandler` now avoids unsafe stringification for scanner-fed dynamic
tokens.  It uses raw `BoxHelpers.map_get` values plus `BoxHelpers.same_token`
for dynamic token equality.

This fixes the same class of AOT string equality issue that affected the
Layer4 loop DTO work, without adding a new `.hako` syntax feature, library API,
backend route, or ABI.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_if_recipe_dto_parity_gate.sh
```

Expected guard result:

```text
owner=ProgramJsonIfRecipeDtoSnapshotV1
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
MIRBUILDER-PROGRAMJSON-LAYER4-IF-RECIPE-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
