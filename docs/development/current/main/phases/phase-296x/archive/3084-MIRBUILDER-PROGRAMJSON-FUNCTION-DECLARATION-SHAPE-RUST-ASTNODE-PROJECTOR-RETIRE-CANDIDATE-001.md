# 3084 - MIRBUILDER-PROGRAMJSON-FUNCTION-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3083 is green, mark only the covered
`FunctionDeclarationShapeSnapshotV1` ProgramJSON traversal rows as a Rust
ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_function_declaration_shape_scan_parity_gate.sh
```

## Covered Rows

```text
fn_no_ret_empty
fn_ret_empty
fn_no_ret_local
fn_ret_return
fn_no_ret_expr
fn_no_ret_loop
fn_ret_if
fn_missing_name_unsupported
fn_body_break_unsupported
first_stmt_local_unsupported
```

## Retire Candidate

```text
FunctionDeclarationShapeSnapshotV1 for covered ProgramJSON FunctionDeclaration rows
```

## Not Retired

- full Rust ASTNode projector;
- full FunctionDeclaration extractor or lowerer;
- function lowering or route selection;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_function_declaration_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-function-declaration-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
