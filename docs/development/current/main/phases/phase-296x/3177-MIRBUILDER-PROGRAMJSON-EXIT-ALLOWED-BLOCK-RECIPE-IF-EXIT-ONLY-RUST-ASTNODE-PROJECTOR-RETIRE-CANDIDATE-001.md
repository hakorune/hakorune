# 3177 - MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-IF-EXIT-ONLY-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1` row
from 3176 as a scoped Rust ASTNode projector retire-candidate.

Covered row:

```text
if_then_return_no_else
```

Deferred rows:

```text
join_then_else
loop_v0_block
exit_allowed_then_else_modes
```

This is a proof checkpoint only. It does not switch the runtime route away from
Rust, remove the bootstrap oracle, execute RecipeMatcher, materialize
RecipeBodies, select backend routes, lower MIR, mutate MIR, allocate IDs, or
claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_exit_allowed_block_recipe_if_exit_only_retire_rust_astnode_projector_candidate_guard.sh
```

Expected guard result:

```text
retire_candidate=ProgramJsonExitAllowedBlockRecipeIfExitOnlySnapshotV1
covered_rows=1
programjson_exit_allowed_if_exit_only_snapshot_parity_gate=green
recipe_root_traversal_used=1
exit_allowed_reducer_called=1
if_exit_only_token_projected=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
```

## Non-Claims

```text
JoinThenElse contract
LoopV0 contract
other ExitAllowed if-modes
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
MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-EXIT-ALLOWED-NEXT-CONTRACT-SELECTION-001
```
