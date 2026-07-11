# 3056 - MIRBUILDER-PROGRAMJSON-FASTMEM-REGION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3055 is parity-green, mark only the covered
`FastMemRegionShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
fastmem lowering or contract execution.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_fastmem_region_shape_scan_parity_gate.sh
```

The 3055 gate must prove:

```text
capability=ProgramJsonFastMemRegionShapeScanV1
output=FastMemRegionShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
fastmem_lowering=0
contract_execution=0
```

## Covered Rows

```text
top_fastmem_empty
top_fastmem_local
top_fastmem_return
top_fastmem_break
top_fastmem_continue
top_fastmem_loop
top_fastmem_expr
if_then_fastmem_local
if_else_fastmem_empty
first_stmt_local_unsupported
```

## Retire Candidate

```text
FastMemRegionShapeSnapshotV1
for covered ProgramJSON FastMemRegion statement body rows
```

## Not Retired

```text
full Rust ASTNode projector
full FastMemRegion extractor
fastmem lowering
contract execution
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

- the 3055 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- fastmem lowering and contract execution remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_fastmem_region_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-fastmem-region-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=FastMemRegionShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
fastmem_lowering=0
contract_execution=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
