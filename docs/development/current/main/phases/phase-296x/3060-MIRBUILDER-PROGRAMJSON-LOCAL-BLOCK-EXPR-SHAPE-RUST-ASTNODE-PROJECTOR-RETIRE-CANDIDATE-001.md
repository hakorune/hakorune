# 3060 - MIRBUILDER-PROGRAMJSON-LOCAL-BLOCK-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3059 is parity-green, mark only the covered
`LocalBlockExprShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
block expression lowering or prelude execution semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_block_expr_shape_scan_parity_gate.sh
```

The 3059 gate must prove:

```text
capability=ProgramJsonLocalBlockExprShapeScanV1
output=LocalBlockExprShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
block_expr_lowering=0
prelude_execution_semantics=0
```

## Covered Rows

```text
top_empty_prelude_tail_int
top_empty_prelude_tail_str
top_empty_prelude_tail_bool
top_local_prelude_tail_int
top_expr_prelude_tail_str
top_return_prelude_tail_var
top_local_expr_prelude_tail_var
if_then_local_prelude_tail_int
if_else_empty_prelude_tail_int
first_stmt_return_unsupported
```

## Retire Candidate

```text
LocalBlockExprShapeSnapshotV1
for covered ProgramJSON Local.expr BlockExpr rows
```

## Not Retired

```text
full Rust ASTNode projector
full Local BlockExpr extractor
block expression lowering
prelude execution semantics
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

- the 3059 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- block expression lowering and prelude execution semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_block_expr_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-local-block-expr-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=LocalBlockExprShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
block_expr_lowering=0
prelude_execution_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
