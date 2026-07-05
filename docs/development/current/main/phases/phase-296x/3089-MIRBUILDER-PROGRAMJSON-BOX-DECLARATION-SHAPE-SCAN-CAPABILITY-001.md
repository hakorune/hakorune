# 3089 - MIRBUILDER-PROGRAMJSON-BOX-DECLARATION-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add real `.hako` ProgramJSON traversal for covered `BoxDeclaration` shapes.
The owner reads the top-level ProgramJSON body, locates a `BoxDeclaration`, then
inspects box kind flags and member arrays.

## Implemented Owner

```text
ProgramJsonBoxDeclarationShapeScanV1
```

Output:

```text
BoxDeclarationShapeSnapshotV1
```

Covered shape kinds:

```text
PlainBox
InterfaceBox
RecordBox
StaticBox
BoxWithFieldDecl
BoxWithMethod
BoxWithConstructor
BoxWithStaticInit
Unsupported
```

## Parity Rows

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

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_box_declaration_shape_scan_parity_gate.sh
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

- box lowering;
- method lowering;
- field layout;
- full Rust ASTNode projector retirement;
- HakoAdoption for a full owner;
- ProgramJSON full parser;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- Source Selfhost;
- new backend route or ABI.

## Next

```text
MIRBUILDER-PROGRAMJSON-BOX-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
