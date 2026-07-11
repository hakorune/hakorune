# 3085 - MIRBUILDER-PROGRAMJSON-TYPE-ALIAS-DECLARATION-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonTypeAliasDeclarationShapeScanV1` as the next ProgramJSON
traversal capability.

The owner consumes ProgramJSON structure and emits a
`TypeAliasDeclarationShapeSnapshotV1` token snapshot for covered top-level
`TypeAliasDeclaration` shapes. It observes alias name presence and target type
category only.

ProgramJSON v0 evidence is `src/macro/ast_json/joinir_compat.rs`, where
`ASTNode::TypeAliasDeclaration` emits `kind: "TypeAliasDeclaration"` with
`name` and `target_type` fields.

## Minimum Rows

```text
alias_i64
alias_string
alias_bool
alias_arraybox
alias_mapbox
alias_custom_other
alias_missing_target_unsupported
first_stmt_local_unsupported
```

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_type_alias_declaration_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonTypeAliasDeclarationShapeScanV1
output_contract=TypeAliasDeclarationShapeSnapshotV1
parity_rows=8
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
type_resolution=0
alias_expansion=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_type_alias_declaration_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-type-alias-declaration-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-TYPE-ALIAS-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- type resolution or alias expansion;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
