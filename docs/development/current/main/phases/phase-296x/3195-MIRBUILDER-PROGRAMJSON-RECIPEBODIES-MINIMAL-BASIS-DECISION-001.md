# 3195 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-DECISION-001

Status: landed

## Decision

Adopt 3194 option A:

```text
A_DTO_ONLY_STMT_ONLY_BODYID_STMTREF_SNAPSHOT
```

The first RecipeBodies-facing `.hako` slice will be a DTO-only,
snapshot-local BodyId/StmtRef proof. It must not publish or construct a real
runtime `RecipeBodies` arena.

## Why

The migration needs to move past flat block recipe token reducers, but the next
boundary is sensitive:

```text
RecipeBlock { body_id, items: Vec<RecipeItem> }
RecipeBodies/RecipeBody internal arena
BodyId/StmtRef reference boundary
```

A DTO-only StmtOnly snapshot proves that `.hako` can represent the reference
surface without opening recursive arena semantics, RecipeMatcher, route
selection, lowering, MIR mutation, or ID allocation.

## Selected First Slice

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001
```

Expected snapshot:

```text
snapshot_kind=ProgramJsonRecipeBodiesMinimalDtoV1
err=0|1
root_body_id=0
body_count=N
body0_item_count=N
body0_items=Stmt:Local,Stmt:Print
refs=body0.item0->stmt0,body0.item1->stmt1
non_claims=recipe_bodies_materialization,lowering,route_selection,id_allocation
```

Contract:

```text
BodyId = snapshot-local token only
StmtRef = snapshot-local token only
```

## First Slice Acceptance

```text
must consume ProgramJSON
must use existing recipe_root or ProgramJSON scan vocabulary
must emit BodyId/StmtRef tokens
must compare against Rust oracle
must name non-claims in output
```

Minimum rows:

```text
empty_stmt_only_body
single_local_stmt_body
local_then_print_stmt_body
```

## Deferred

```text
B_ONE_SHAPE_ARENA_BUILDER_PARITY
C_VERIFIER_FIRST_BOUNDARY
```

These remain deferred until the DTO boundary has parity or a new explicit
decision changes the order.

## Non-Claims

```text
RecipeBodies materialization
runtime RecipeBodies arena
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_minimal_basis_decision_guard.sh
```

Expected result:

```text
selected_option=A_DTO_ONLY_STMT_ONLY_BODYID_STMTREF_SNAPSHOT
selected_first_slice=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001
recipe_bodies_materialization=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001
```
