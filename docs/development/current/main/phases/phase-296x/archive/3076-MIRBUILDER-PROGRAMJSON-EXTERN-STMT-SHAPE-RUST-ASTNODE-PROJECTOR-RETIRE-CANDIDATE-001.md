# 3076 - MIRBUILDER-PROGRAMJSON-EXTERN-STMT-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3075 is green, mark only the covered `ExternStmtShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_extern_stmt_shape_scan_parity_gate.sh
```

## Covered Rows

```text
extern_console_log_int_arg
extern_console_log_var_arg
extern_console_warn_str_arg
extern_console_error_bool_arg
extern_console_log_no_arg
extern_console_log_two_arg_unsupported
extern_env_time_unsupported
if_then_extern_console_log_int_arg
if_else_extern_console_warn_str_arg
expr_call_unsupported
```

## Retire Candidate

```text
ExternStmtShapeSnapshotV1 for covered ProgramJSON Extern statement rows
```

## Not Retired

- full Rust ASTNode projector;
- full Extern statement extractor;
- extern route lowering or route publication;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Acceptance

- retire fixture names only the covered extern-stmt rows;
- guard requires 3075 parity green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only;
- extern route lowering and route publication remain unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_extern_stmt_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-extern-stmt-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
