# 3221 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

Mark the expanded `ProgramJsonRecipeBodiesVerifierBoundarySnapshotV1` DTO rows
as scoped Rust ASTNode projector retire-candidates.

Covered rows:

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
```

This means the ProgramJSON route can build the covered RecipeBodies arena DTOs
and validate them through the existing `RecipeVerifierBox.verify/2` result-map
boundary.

It does not mean runtime route switching, runtime `RecipeBodies` publication,
full RecipeMatcher execution, or full Rust projector removal.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_verifier_boundary_expanded_dto_coverage_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_verifier_boundary_expanded_dto_coverage_retire_rust_astnode_projector_candidate_guard.sh
```

Expected result:

```text
retire_candidate_recorded=1
covered_row_count=2
rust_projector_runtime_dependency_removed=0
full_astnode_projector_retired=0
recipe_bodies_verifier_boundary_implemented=1
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
DirectAbi route publication expansion
runtime route switch
ProgramJSON full parser
new backend route
new ABI
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-EXPANDED-DTO-COVERAGE-NEXT-CONTRACT-SELECTION-001
```
