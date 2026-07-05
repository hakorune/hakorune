# 3075 - MIRBUILDER-PROGRAMJSON-EXTERN-STMT-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExternStmtShapeScanV1` as the next ProgramJSON traversal
capability.

The owner consumes ProgramJSON structure and emits an
`ExternStmtShapeSnapshotV1` token snapshot for covered `Extern` statement
shapes.

## Minimum Rows

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

## Required Output

```text
snapshot_kind=ExternStmtShapeSnapshotV1
top_extern_stmt_shape_kind=...
if_then_extern_stmt_shape_kind=...
if_else_extern_stmt_shape_kind=...
supported_extern_stmt_count=...
unsupported_extern_stmt_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Extern` statement positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported extern shapes are reported with a stable token;
- the card can name a concrete extern-stmt Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_extern_stmt_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonExternStmtShapeScanV1
output_contract=ExternStmtShapeSnapshotV1
parity_rows=10
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
extern_route_lowering=0
extern_route_publication=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_extern_stmt_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-extern-stmt-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-EXTERN-STMT-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- extern route lowering or route publication;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
