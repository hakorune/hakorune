# 3008 - MIRBUILDER-PROGRAMJSON-LOCAL-BINDING-SHAPE-SCAN-CAPABILITY-001

Status: active

## Scope

Implement `ProgramJsonLocalBindingShapeScanV1` as the next ProgramJSON
traversal capability.

The owner must consume ProgramJSON structure and emit a
`LocalBindingShapeSnapshotV1` token snapshot for covered local-binding shapes.

## Minimum Rows

```text
local_int_then_return_var
local_bool_then_return_var
local_var_alias_then_return_var
local_compare_var_lt_int_then_return_var
local_compare_var_eq_int_then_return_var
local_call_unsupported_then_return_var
if_then_local_int_then_return_var
if_else_local_var_then_return_var
```

## Required Output

```text
snapshot_kind=LocalBindingShapeSnapshotV1
top_local_init_kind=...
if_then_local_init_kind=...
if_else_local_init_kind=...
supported_local_init_count=...
unsupported_local_init_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered local
  binding positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported local initializers are reported with a stable token;
- the card can name a concrete local-binding Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
