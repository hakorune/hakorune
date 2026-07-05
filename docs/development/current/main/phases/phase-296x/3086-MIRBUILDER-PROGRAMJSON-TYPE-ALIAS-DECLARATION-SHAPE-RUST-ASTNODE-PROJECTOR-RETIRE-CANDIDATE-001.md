# 3086 - MIRBUILDER-PROGRAMJSON-TYPE-ALIAS-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3085 is green, mark only the covered
`TypeAliasDeclarationShapeSnapshotV1` ProgramJSON traversal rows as a Rust
ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_type_alias_declaration_shape_scan_parity_gate.sh
```

## Covered Rows

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

## Retire Candidate

```text
TypeAliasDeclarationShapeSnapshotV1 for covered ProgramJSON TypeAliasDeclaration rows
```

## Not Retired

- full Rust ASTNode projector;
- full TypeAliasDeclaration extractor or lowerer;
- type resolution or alias expansion;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_type_alias_declaration_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-type-alias-declaration-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
