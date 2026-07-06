# 3198 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonRecipeBodiesMinimalDtoV1` rows as a scoped Rust
ASTNode projector retire-candidate.

Covered rows:

```text
empty_stmt_only_body
single_local_stmt_body
local_then_print_stmt_body
```

This means the ProgramJSON route has parity for the DTO-only
snapshot-local BodyId/StmtRef surface. It does not mean runtime route switching
or removal of the full Rust projector.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_minimal_dto_snapshot_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_minimal_dto_retire_rust_astnode_projector_candidate_guard.sh
```

Expected result:

```text
retire_candidate_recorded=1
rust_projector_runtime_dependency_removed=0
full_astnode_projector_retired=0
recipe_bodies_materialization=0
source_selfhost_claim=0
```

## Non-Claims

```text
runtime dependency removal
full Rust ASTNode projector retirement
RecipeBodies materialization
runtime RecipeBodies arena
RecipeBodies::bodies access
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
DirectAbi route publication
runtime route switch
ProgramJSON full parser
new backend route
new ABI
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-MINIMAL-DTO-NEXT-CONTRACT-SELECTION-001
```
