# 3088 - MIRBUILDER-PROGRAMJSON-ENUM-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3087 is green, mark only the covered
`EnumDeclarationShapeSnapshotV1` ProgramJSON traversal rows as a Rust ASTNode
projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_declaration_shape_scan_parity_gate.sh
```

## Covered Rows

```text
enum_empty_variants
enum_unit_first_variant
enum_payload_first_variant
enum_tuple_first_variant
enum_record_first_variant
enum_missing_name_unsupported
enum_missing_variant_name_unsupported
first_stmt_type_alias_unsupported
```

## Retire Candidate

```text
EnumDeclarationShapeSnapshotV1 for covered ProgramJSON EnumDeclaration rows
```

## Not Retired

- full Rust ASTNode projector;
- full EnumDeclaration extractor or lowerer;
- enum lowering or variant resolution;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_declaration_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-enum-declaration-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
