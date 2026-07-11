# 3068 - MIRBUILDER-PROGRAMJSON-NEW-FIELD-INITIALIZER-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3067 is parity-green, mark only the covered
`NewFieldInitializerShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode
projector retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
box field initializer lowering or object allocation.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_new_field_initializer_shape_scan_parity_gate.sh
```

The 3067 gate must prove:

```text
capability=ProgramJsonNewFieldInitializerShapeScanV1
output=NewFieldInitializerShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
box_field_initializer_lowering=0
object_allocation=0
```

## Covered Rows

```text
local_no_field_initializers
local_one_int_field_init
local_one_str_field_init
local_one_bool_field_init
local_one_var_field_init
expr_one_new_field_init
return_two_int_field_inits
local_two_var_field_inits
if_then_one_int_field_init
if_else_one_var_field_init
```

## Retire Candidate

```text
NewFieldInitializerShapeSnapshotV1
for covered ProgramJSON New.field_initializers rows
```

## Not Retired

```text
full Rust ASTNode projector
full New field-initializer extractor
box field initializer lowering
object allocation
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

- the 3067 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- box field initializer lowering and object allocation remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_new_field_initializer_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-new-field-initializer-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=NewFieldInitializerShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
box_field_initializer_lowering=0
object_allocation=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
