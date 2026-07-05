# 3009 - MIRBUILDER-PROGRAMJSON-LOCAL-BINDING-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

Mark only the covered `LocalBindingShapeSnapshotV1` ProgramJSON traversal rows
as a Rust ASTNode projector retire-candidate.

Covered rows:

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

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_binding_shape_scan_parity_gate.sh
```

## Retire Candidate

```text
Rust ASTNode projector slice:
  LocalBindingShapeSnapshotV1 for covered Local.expr ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full local binding extractor
full loop_cond_continue_with_return facts extractor
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption for a full owner
Source Selfhost
```

## Acceptance

- retire-candidate fixture names only the covered local-binding rows;
- guard requires the 3008 ProgramJSON local-binding parity gate to be green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only for this covered slice.
