# 3196 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-SELECTION-001

Status: landed

## Scope

Select the `.hako` owner for the first RecipeBodies-facing DTO snapshot.

Selected owner:

```text
lang/src/compiler/mirbuilder/program_json_recipebodies_minimal_dto_snapshot.hako
ProgramJsonRecipeBodiesMinimalDtoSnapshotBox
```

This owner is separate from the existing block recipe token reducers. It emits
only a DTO snapshot of snapshot-local `BodyId`/`StmtRef` tokens.

## Selected Rows

```text
empty_stmt_only_body
single_local_stmt_body
local_then_print_stmt_body
```

Expected output kind:

```text
ProgramJsonRecipeBodiesMinimalDtoV1
```

Required fields:

```text
snapshot_kind
err
root_body_id
body_count
body0_item_count
body0_items
refs
non_claims
```

## Contract

```text
BodyId = snapshot-local token only
StmtRef = snapshot-local token only
```

This is not a real `RecipeBodies` arena and must not expose
`RecipeBodies::bodies`.

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-PARITY-001
```

That next card is allowed to add the `.hako` owner and parity gate.

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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_minimal_dto_snapshot_selection_guard.sh
```

Expected result:

```text
selected_owner=ProgramJsonRecipeBodiesMinimalDtoSnapshotBox
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-PARITY-001
recipe_bodies_materialization=0
source_selfhost_claim=0
```
