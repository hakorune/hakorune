# 3032 - MIRBUILDER-PROGRAMJSON-THROW-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

After 3031 is parity-green, mark only the covered
`ThrowExprShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
exception runtime semantics or catch/finally matching.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_throw_expr_shape_scan_parity_gate.sh
```

The 3031 gate must prove:

```text
capability=ProgramJsonThrowExprShapeScanV1
output=ThrowExprShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
exception_runtime_semantics=0
catch_finally_matching=0
```

## Covered Rows

```text
top_throw_int
top_throw_var
top_throw_str
top_throw_bool_true
top_throw_compare_var_lt_int
top_throw_call_unsupported
if_then_throw_int
if_else_throw_var
first_stmt_not_throw_unsupported
```

## Retire Candidate

```text
ThrowExprShapeSnapshotV1
for covered ProgramJSON Throw.expr rows
```

## Not Retired

```text
full Rust ASTNode projector
full throw statement extractor
exception runtime semantics
catch/finally matching
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption
Source Selfhost
new ABI
```

## Acceptance

- the 3031 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- exception runtime semantics and catch/finally matching remain explicitly
  unclaimed.
