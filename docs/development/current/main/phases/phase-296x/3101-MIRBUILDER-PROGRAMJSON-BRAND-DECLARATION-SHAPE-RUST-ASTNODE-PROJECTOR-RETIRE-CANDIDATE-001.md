# 3101 - MIRBUILDER-PROGRAMJSON-BRAND-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `BrandDeclarationShapeSnapshotV1` ProgramJSON traversal
rows as a scoped Rust ASTNode projector retire-candidate after 3100 parity is
green.

This does not retire the full Rust ASTNode projector and does not add brand
type resolution or brand lowering.

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_brand_declaration_shape_scan_parity_gate.sh
```

## Retire Candidate Scope

```text
BrandDeclarationShapeSnapshotV1
for covered ProgramJSON BrandDeclaration rows
```

Covered rows:

```text
brand_i64
brand_string
brand_bool
brand_arraybox
brand_mapbox
brand_custom_other
brand_missing_underlying_unsupported
first_stmt_local_unsupported
```

## Not Retired

```text
full Rust ASTNode projector
brand type resolution
brand lowering
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_brand_declaration_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Guard result:

```text
decision=RetireCandidateScoped
parity_gate=green
covered_rows=8
brand_type_resolution=0
brand_lowering=0
full_astnode_projector_retired=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
