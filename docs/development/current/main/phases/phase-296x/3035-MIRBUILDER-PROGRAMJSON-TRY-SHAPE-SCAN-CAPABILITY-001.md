# 3035 - MIRBUILDER-PROGRAMJSON-TRY-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonTryShapeScanV1` as the next ProgramJSON traversal
capability.

The owner consumes ProgramJSON structure and emits a `TryShapeSnapshotV1` token
snapshot for covered `Try` statement shapes.

## Minimum Rows

```text
top_try_throw_no_catch_no_finally
top_try_return_one_catch_no_finally
top_try_expr_one_catch_finally_expr
top_try_empty_many_catches_no_finally
top_try_return_no_catch_finally_return
if_then_try_throw_no_catch_no_finally
if_else_try_return_one_catch_no_finally
top_try_nested_try_unsupported
first_stmt_not_try_unsupported
```

## Required Output

```text
snapshot_kind=TryShapeSnapshotV1
top_try_shape_kind=...
if_then_try_shape_kind=...
if_else_try_shape_kind=...
supported_try_count=...
unsupported_try_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Try.try`, `Try.catches`, and `Try.finally` positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported nested Try / non-Try shapes are reported with stable tokens;
- the card can name a concrete Try Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- exception runtime semantics, catch matching, or finally execution semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_try_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonTryShapeScanV1
output_contract=TryShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
exception_runtime_semantics=0
catch_matching=0
finally_execution_semantics=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_try_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-try-shape-scan-parity-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-TRY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
