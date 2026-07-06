# 3201 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-ONE-SHAPE-ARENA-BUILDER-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonRecipeBodiesOneShapeArenaBuilderV1` rows as a
scoped Rust ASTNode projector retire-candidate.

Covered rows:

```text
empty_stmt_only_body
single_local_stmt_body
local_then_print_stmt_body
```

This means the ProgramJSON route can build and summarize a map-backed
one-shape arena DTO for these rows. It does not mean runtime route switching,
runtime `RecipeBodies` publication, or full Rust projector removal.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_one_shape_arena_builder_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_one_shape_arena_builder_retire_rust_astnode_projector_candidate_guard.sh
```

Expected result:

```text
retire_candidate_recorded=1
rust_projector_runtime_dependency_removed=0
full_astnode_projector_retired=0
recipe_bodies_materialization=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Non-Claims

```text
runtime dependency removal
full Rust ASTNode projector retirement
runtime RecipeBodies arena
RecipeBodies::bodies access
full RecipeMatcher execution
verifier policy reimplementation
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
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-ONE-SHAPE-ARENA-NEXT-CONTRACT-SELECTION-001
```
