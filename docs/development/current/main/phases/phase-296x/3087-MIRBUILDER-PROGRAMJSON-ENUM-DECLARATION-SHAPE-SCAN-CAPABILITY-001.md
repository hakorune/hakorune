# 3087 - MIRBUILDER-PROGRAMJSON-ENUM-DECLARATION-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add real `.hako` ProgramJSON traversal for covered `EnumDeclaration` shapes.
The owner reads the top-level ProgramJSON body, locates an `EnumDeclaration`,
then inspects the `variants` array and first variant shape.

## Implemented Owner

```text
ProgramJsonEnumDeclarationShapeScanV1
```

Output:

```text
EnumDeclarationShapeSnapshotV1
```

Covered shape kinds:

```text
EnumNoVariants
EnumFirstUnitVariant
EnumFirstPayloadVariant
EnumFirstTupleVariant
EnumFirstRecordVariant
EnumFirstUnsupported
Unsupported
```

## Parity Rows

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

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_declaration_shape_scan_parity_gate.sh
```

Gate result:

```text
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
```

## Explicit Non-Claims

- enum lowering;
- variant resolution;
- full Rust ASTNode projector retirement;
- HakoAdoption for a full owner;
- ProgramJSON full parser;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- Source Selfhost;
- new backend route or ABI.

## Next

```text
MIRBUILDER-PROGRAMJSON-ENUM-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
