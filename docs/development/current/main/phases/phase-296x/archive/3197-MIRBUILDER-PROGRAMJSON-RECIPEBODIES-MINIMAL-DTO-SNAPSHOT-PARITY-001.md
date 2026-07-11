# 3197 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-SNAPSHOT-PARITY-001

Status: landed

## Scope

Implement and prove the first DTO-only RecipeBodies-facing ProgramJSON snapshot:

```text
ProgramJsonRecipeBodiesMinimalDtoV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_recipebodies_minimal_dto_snapshot.hako
```

Rows:

```text
empty_stmt_only_body
single_local_stmt_body
local_then_print_stmt_body
```

## Contract

The output uses snapshot-local reference tokens only:

```text
root_body_id=0
refs=body0.item0->stmt0,...
```

This is not a runtime `RecipeBodies` arena and does not expose
`RecipeBodies::bodies`.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_minimal_dto_snapshot_parity_gate.sh
```

Expected result:

```text
runtime_parity_green=1
directabi_route_publication_claim=0
recipe_bodies_materialization=0
source_selfhost_claim=0
```

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

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
