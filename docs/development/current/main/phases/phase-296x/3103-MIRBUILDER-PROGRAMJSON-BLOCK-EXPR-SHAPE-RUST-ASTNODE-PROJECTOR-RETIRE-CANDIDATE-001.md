# 3103 - MIRBUILDER-PROGRAMJSON-BLOCK-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `BlockExprShapeSnapshotV1` ProgramJSON traversal rows as
a scoped Rust ASTNode projector retire-candidate after 3102 parity is green.

This does not retire the full Rust ASTNode projector and does not add block
expression lowering or prelude execution semantics.

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_expr_shape_scan_parity_gate.sh
```

## Retire Candidate Scope

```text
BlockExprShapeSnapshotV1
for covered ProgramJSON top-level BlockExpr rows
```

Covered rows:

```text
empty_prelude_tail_int
empty_prelude_tail_string
empty_prelude_tail_bool
local_prelude_tail_int
expr_prelude_tail_string
return_prelude_tail_var
local_expr_prelude_tail_var
many_prelude_tail_int
tail_unsupported
first_stmt_local_unsupported
```

## Not Retired

```text
full Rust ASTNode projector
block expression lowering
prelude execution semantics
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption for a full owner
Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_expr_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Guard result:

```text
decision=RetireCandidateScoped
parity_gate=green
covered_rows=10
block_expr_lowering=0
prelude_execution_semantics=0
full_astnode_projector_retired=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
