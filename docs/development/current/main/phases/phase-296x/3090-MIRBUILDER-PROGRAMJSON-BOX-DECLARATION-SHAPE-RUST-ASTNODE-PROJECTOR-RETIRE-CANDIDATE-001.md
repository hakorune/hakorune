# 3090 - MIRBUILDER-PROGRAMJSON-BOX-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3089 is green, mark only the covered
`BoxDeclarationShapeSnapshotV1` ProgramJSON traversal rows as a Rust ASTNode
projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_box_declaration_shape_scan_parity_gate.sh
```

## Covered Rows

```text
plain_box
interface_box
record_box
static_box
box_with_field_decl
box_with_method
box_with_constructor
box_with_static_init
first_stmt_enum_unsupported
```

## Retire Candidate

```text
BoxDeclarationShapeSnapshotV1 for covered ProgramJSON BoxDeclaration rows
```

## Not Retired

- full Rust ASTNode projector;
- full BoxDeclaration extractor or lowerer;
- box lowering, method lowering, or field layout;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_box_declaration_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-box-declaration-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
