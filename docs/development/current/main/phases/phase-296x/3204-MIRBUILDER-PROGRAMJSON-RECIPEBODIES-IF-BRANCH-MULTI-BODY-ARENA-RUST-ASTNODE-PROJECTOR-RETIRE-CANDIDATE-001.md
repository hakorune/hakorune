# 3204 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-IF-BRANCH-MULTI-BODY-ARENA-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonRecipeBodiesIfBranchArenaBuilderV1` row as a
scoped Rust ASTNode projector retire-candidate.

Covered row:

```text
local_if_then_local_else_print_return
```

This means the ProgramJSON route can build and summarize a map-backed
If-branch multi-body arena DTO for this row:

```text
body0 = [Stmt:Local, IfRef(then=1, else=2), Stmt:Return]
body1 = [Stmt:Local]
body2 = [Stmt:Print]
```

It does not mean runtime route switching, runtime `RecipeBodies` publication,
or full Rust projector removal.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_if_branch_multi_body_arena_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_if_branch_multi_body_arena_retire_rust_astnode_projector_candidate_guard.sh
```

Expected result:

```text
retire_candidate_recorded=1
rust_projector_runtime_dependency_removed=0
full_astnode_projector_retired=0
body_count=3
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
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-IF-BRANCH-ARENA-NEXT-CONTRACT-SELECTION-001
```
