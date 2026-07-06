# 3132 - MIRBUILDER-PROGRAMJSON-LAYER4-EXIT-RECIPE-DTO-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ExitRecipeDtoSnapshotV1` ProgramJSON Layer4 rows from 3131 as
a scoped Rust ASTNode projector retire-candidate.

This is a future runtime route-switch candidate only.  Rust remains the
bootstrap/oracle path until a later route-switch card proves the self-contained
HHako path.

## Covered Rows

```text
local_if_then_return_int_final_return_int
local_if_then_return_int_final_return_var
local_if_then_else_assignment_no_exit_reject
```

Deferred:

```text
loop_body_exit
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_layer4_exit_recipe_dto_retire_rust_astnode_projector_candidate_guard.sh
```

Expected guard result:

```text
retire_candidate=ExitRecipeDtoSnapshotV1
covered_rows=3
deferred_rows=loop_body_exit
decision=RetireCandidateScoped
parity_gate=green
programjson_runtime_parity_green=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering=0
source_selfhost_claim=0
```

## Non-Claims

```text
loop-body Exit DTO
runtime route switch
full ASTNode projector retirement
full RecipeMatcher execution
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-NEXT-RECIPE-DTO-CAPABILITY-SELECTION-001
```
