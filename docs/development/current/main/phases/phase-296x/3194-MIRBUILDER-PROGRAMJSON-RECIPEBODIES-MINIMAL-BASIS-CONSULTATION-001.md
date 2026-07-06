# 3194 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001

Status: landed

## Scope

Prepare the design consultation required by 3193 before implementing any
RecipeBodies surface in `.hako`.

The flat ProgramJSON block recipe rows now cover JoinThenElse, ElseOnlyExit,
ThenOnlyExit, ExitAll, and LoopExitAllowedBody. The next step is no longer
another string token reducer. It is the first boundary where `.hako` would need
to represent the Rust `RecipeBlock { body_id, items }` shape and
`BodyId`/`StmtRef` reference surface.

## Current Evidence

Rust authority:

```text
src/mir/builder/control_flow/recipes/body.rs
src/mir/builder/control_flow/recipes/refs.rs
src/mir/builder/control_flow/plan/recipe_tree/block.rs
src/mir/builder/control_flow/plan/recipe_tree/mod.rs
src/mir/builder/control_flow/plan/recipe_tree/verified.rs
src/mir/builder/control_flow/plan/parts/entry.rs
src/mir/builder/control_flow/plan/parts/verify.rs
```

`.hako` current state:

```text
lang/src/compiler/mirbuilder/recipe/recipe_item_box.hako
lang/src/compiler/mirbuilder/recipe/recipe_verifier_box.hako
lang/src/compiler/mirbuilder/program_json_v0_phase_state_box.hako
lang/src/compiler/mirbuilder/program_json_no_exit_block_recipe_snapshot.hako
lang/src/compiler/mirbuilder/program_json_exit_allowed_block_recipe_snapshot.hako
```

`.hako` currently emits inline `RecipeItem` maps and snapshot token summaries.
It does not materialize `RecipeBodies`, `RecipeBody`, `BodyId`, or `StmtRef`.

## Consultation Question

```text
Which minimal RecipeBodies surface should be exposed to .hako first after the
flat block recipe rows:

A. DTO-only snapshot-local BodyId/StmtRef
B. one-shape arena builder parity
C. verifier-first boundary
```

Recommended answer:

```text
A. DTO-only snapshot-local BodyId/StmtRef
```

Reason:

```text
StmtOnly DTO proves the reference boundary without exposing RecipeBodies
internals, recursive arena semantics, RecipeMatcher, route selection, lowering,
mutation, or ID allocation.
```

## Recommended First Slice

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001
```

Shape:

```text
snapshot_kind=ProgramJsonRecipeBodiesMinimalDtoV1
err=0|1
root_body_id=0
body_count=N
body0_item_count=N
body0_items=Stmt:Local,Stmt:Print
refs=body0.item0->stmt0,body0.item1->stmt1
```

Important wording:

```text
BodyId and StmtRef in this first slice are snapshot-local tokens only.
They are not public RecipeBodies arena IDs.
```

## Non-Claims

```text
RecipeBodies materialization
RecipeBodies::bodies access
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
ProgramJSON full parser
new backend route
new ABI
Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_minimal_basis_consultation_guard.sh
```

Expected result:

```text
consultation_prepared=1
recommended_option=A_DTO_ONLY_STMT_ONLY_BODYID_STMTREF_SNAPSHOT
implementation_selected=0
recipe_bodies_materialization=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-DECISION-001
```
