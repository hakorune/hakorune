# 3064 - MIRBUILDER-PROGRAMJSON-BRAND-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3063 is parity-green, mark only the covered
`BrandExprShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
brand lowering or brand runtime semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_brand_expr_shape_scan_parity_gate.sh
```

The 3063 gate must prove:

```text
capability=ProgramJsonBrandExprShapeScanV1
output=BrandExprShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
brand_lowering=0
brand_runtime_semantics=0
```

## Covered Rows

```text
local_construct_i64_from_int
local_construct_string_from_str
local_construct_bool_from_bool
local_construct_from_var
local_unwrap_i64_from_var
local_unwrap_string_from_var
expr_unwrap_bool_from_var
if_then_construct_i64_from_int
if_else_unwrap_i64_from_var
first_stmt_return_unsupported
```

## Retire Candidate

```text
BrandExprShapeSnapshotV1
for covered ProgramJSON BrandConstruct/BrandUnwrap expression rows
```

## Not Retired

```text
full Rust ASTNode projector
full Brand expression extractor
brand lowering
brand runtime semantics
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

- the 3063 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- brand lowering and brand runtime semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_brand_expr_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-brand-expr-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=BrandExprShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
brand_lowering=0
brand_runtime_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
