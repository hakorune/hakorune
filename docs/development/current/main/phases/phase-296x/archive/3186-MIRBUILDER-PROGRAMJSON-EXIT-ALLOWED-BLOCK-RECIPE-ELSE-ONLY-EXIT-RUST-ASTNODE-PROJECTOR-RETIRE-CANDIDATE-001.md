# 3186 - MIRBUILDER-PROGRAMJSON-EXIT-ALLOWED-BLOCK-RECIPE-ELSE-ONLY-EXIT-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark the covered `ProgramJsonExitAllowedBlockRecipeElseOnlyExitSnapshotV1` row
from 3185 as a scoped Rust ASTNode projector retire-candidate.

Covered row:

```text
local_if_then_local_else_return_final_return_var
```

Deferred rows:

```text
ThenOnlyExit
ExitAll
RecipeBodies materialization
```

This is a proof checkpoint only. It does not switch the runtime route away from
Rust, remove the bootstrap oracle, execute RecipeMatcher, materialize
RecipeBodies, select backend routes, lower MIR, mutate MIR, allocate IDs, or
claim Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_exit_allowed_block_recipe_else_only_exit_retire_rust_astnode_projector_candidate_guard.sh
```

Expected guard result:

```text
retire_candidate=ProgramJsonExitAllowedBlockRecipeElseOnlyExitSnapshotV1
covered_rows=1
programjson_else_only_exit_snapshot_parity_gate=green
recipe_root_traversal_used=1
if_stmt_handler_producer_extended=1
exit_allowed_reducer_called=1
else_only_exit_token_projected=1
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
```

## Non-Claims

```text
ThenOnlyExit
ExitAll
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
MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-AFTER-ELSE-ONLY-EXIT-NEXT-CONTRACT-SELECTION-001
```
