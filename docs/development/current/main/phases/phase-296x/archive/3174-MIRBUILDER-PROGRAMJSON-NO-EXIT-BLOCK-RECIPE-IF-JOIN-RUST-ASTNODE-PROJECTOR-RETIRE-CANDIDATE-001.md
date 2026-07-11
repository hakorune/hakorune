# 3174 - MIRBUILDER-PROGRAMJSON-NO-EXIT-BLOCK-RECIPE-IF-JOIN-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1` row from 3173
as a scoped Rust ASTNode projector retire-candidate.

Covered row:

```text
if_then_local_no_else
```

Deferred rows:

```text
join_then_else
exit_allowed_block
loop_v0_block
```

This is a proof checkpoint only. It does not switch the runtime route away from
Rust, remove the bootstrap oracle, execute RecipeMatcher, materialize
RecipeBodies, select backend routes, lower MIR, mutate MIR, allocate IDs, or
claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_no_exit_block_recipe_if_join_retire_rust_astnode_projector_candidate_guard.sh
```

Expected guard result:

```text
retire_candidate=ProgramJsonNoExitBlockRecipeIfJoinSnapshotV1
covered_rows=1
deferred_rows=join_then_else,exit_allowed_block,loop_v0_block
programjson_no_exit_if_join_snapshot_parity_gate=green
programjson_runtime_parity_green=1
recipe_root_traversal_used=1
no_exit_reducer_called=1
if_join_token_projected=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
```

## Non-Claims

```text
JoinThenElse contract
ExitAllowed contract
LoopV0 contract
RecipeBodies materialization
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
MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-NEXT-CONTRACT-CAPABILITY-SELECTION-001
```
