# 3066 - MIRBUILDER-PROGRAMJSON-ENUM-CTOR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3065 is parity-green, mark only the covered
`EnumCtorShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
enum lowering or payload ABI materialization.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_ctor_shape_scan_parity_gate.sh
```

The 3065 gate must prove:

```text
capability=ProgramJsonEnumCtorShapeScanV1
output=EnumCtorShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
enum_lowering=0
payload_abi_materialization=0
```

## Covered Rows

```text
local_no_payload
local_one_int_payload
local_one_str_payload
local_one_bool_payload
local_one_var_payload
return_two_int_payloads
expr_two_var_payloads
local_compat_box_one_payload
if_then_one_int_payload
if_else_one_var_payload
```

## Retire Candidate

```text
EnumCtorShapeSnapshotV1
for covered ProgramJSON EnumCtor expression rows
```

## Not Retired

```text
full Rust ASTNode projector
full EnumCtor extractor
enum lowering
payload ABI materialization
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

- the 3065 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- enum lowering and payload ABI materialization remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_ctor_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-enum-ctor-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=EnumCtorShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
enum_lowering=0
payload_abi_materialization=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
