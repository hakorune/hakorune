# 3170 - MIRBUILDER-PROGRAMJSON-IF-HANDLER-THEN-LOCAL-NO-ELSE-CAPABILITY-001

Status: landed

## Scope

Extend `IfStmtHandler` with a narrow then-local/no-else capability:

```text
If(cond = Var == Int, then = [Local(Int)], else = null)
```

The resulting Recipe item is an `If` with `then_item = Seq(Local)` and
`else_item = Seq([])`. The existing `ProgramJsonStmtOnlyBlockRecipeSnapshotV1`
projection maps this no-exit If to `IfNoExit`, then calls the existing
HakoAdopted `StmtOnlyBlockRecipeBox` reducer.

Covered row:

```text
local_if_then_local_no_else
```

Expected summary:

```text
snapshot_kind=ProgramJsonStmtOnlyBlockRecipeSnapshotV1;err=0;accepted=1;block_contract=StmtOnly;stmt_count=2;stmt_kinds=Local,If
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_if_handler_then_local_no_else_capability_gate.sh
```

Expected guard result:

```text
owner=IfStmtHandler
row_count=1
programjson_traversal_used=1
if_handler_then_local_no_else_supported=1
recipe_root_traversal_used=1
stmt_only_reducer_called=1
if_no_exit_token_projected=1
prebuilt_token_snapshot_input=0
string_only_facade=0
mir_json_route_green=1
runtime_parity_green=1
source_selfhost_claim=0
```

## Non-Claims

```text
NoExit block contract
ExitAllowed block contract
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
MIRBUILDER-PROGRAMJSON-IF-HANDLER-THEN-LOCAL-NO-ELSE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
