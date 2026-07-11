# 3482 - LANGV1-TYPE-GUARANTEE-PARAMETER-ENTRY-EXACT-NUMERIC-CONTRACT-001

## Status

Complete substantive implementation card after 3481 accepts the callee-entry
owner, typed carrier, exact-numeric subset, and backend fail-fast boundary.

Decision: accepted by 3481.

Implementation: complete.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Objective

Activate the Language v1 semantic contract for explicit exact-numeric source
parameters at one final-callee entry owner. Validate runtime values before
parameter binding and body effects, transport the same typed rows through MIR
JSON, and reject unsupported product backends before execution.

## Single Owner

```text
FunctionEntryContractOwner:
  input  = final MirFunction + runtime argument vector
  output = validated arguments or stable fail-fast error
  order  = exact arity -> row consistency -> runtime value check
```

All MIR-defined direct, nested, recursive, operator, and user-method calls that
reach `exec_function_inner` use this owner. The owner runs after
`pre_exec_reroute`; a rerouted MIR function therefore validates its own rows.
Callers never authoritatively accept an argument.

## Typed Carrier

Add one MIR-owned typed carrier:

```text
FunctionMetadata.parameter_entry_contracts: Vec<ParameterEntryContract>

ParameterEntryContract {
  contract_id
  formal_parameter_index
  source_parameter_index
  parameter_value_id
  source_parameter_name
  declared_type_name
  contract_kind = ExactNumeric
  implicit_receiver = false
  runtime_check_required = true
  proof_elision_allowed = false
  backend_capability_required = parameter_entry_exact_numeric
}
```

Generate rows only for explicit source parameters whose declared type belongs
to the existing exact-numeric type family. Validate index, ValueId, name, and
declared type against the owning function. Missing, duplicate, extra, or drifted
rows fail fast. The implicit method receiver `me` is never emitted.

Keep representation and semantic truth separate:

```text
source :T -> ParameterEntryContract semantic intent
FunctionSignature.params / MirType -> derived callable representation fact
exact_numeric_value_facts -> post-entry callee-body fact only
```

Refactor the current declared-signature builder seam enough to make those two
projections explicit and correct its stale contract comment. Do not broaden or
remove the existing representation projection in this card.

## Ordered Implementation Tasks

### 1. Carrier and construction owner

- define `ParameterEntryContract` beside the existing type-contract MIR data;
- add `FunctionMetadata.parameter_entry_contracts`;
- build rows from `declared_param_decls` through one constructor/validator;
- exclude implicit `me`, extern/FFI, and non-exact-numeric annotations;
- rebuild rows through the normal semantic-refresh lifecycle without a cache;
- add unit tests for row construction and drift.

### 2. Final-callee VM entry owner

- add one `FunctionEntryContractOwner` helper used by `exec_function_inner`;
- invoke it after method reroute and before register state/binding/body effects;
- when at least one active row exists, require exact arity for the complete
  final-callee formal parameter vector;
- retain legacy missing-to-Void/extra-ignore behavior only for functions with
  zero active rows;
- check each incoming exact-numeric runtime value using the existing numeric
  substrate vocabulary;
- seed callee-body exact-numeric facts only after successful validation;
- do not add caller-side checks or proof-elision paths.

### 3. MIR JSON transport

- export declared parameter evidence and typed
  `metadata.parameter_entry_contracts[]` rows;
- preserve formal index and parameter ValueId explicitly;
- add round-trip/export fixtures for accepted, missing, malformed, and drifted
  carriers;
- forbid backend inference from names, position alone, or `MirType` alone.

### 4. Central backend capability

- add `parameter_entry_exact_numeric` inspection to
  `mir::backend_capability::enforce_mir_backend_supported`;
- mark the Rust MIR interpreter as the sole first-slice consumer;
- reject PyVM, LLVM/AOT, and Wasm product execution before effects while an
  active row exists;
- do not route unsupported products through VM;
- do not extend `_seed_hakocli_args_array_fact` or any function-name heuristic.

### 5. Focused closeout

- run carrier, VM entry, MIR JSON, and backend-capability unit tests;
- run direct, nested, recursive, and rerouted final-callee fixtures;
- run the current pointer guard, `cargo check -q`, and the active quick gate;
- record any unrelated full-suite baseline failures without claiming them
  green or changing production defaults.

## Fail-Fast Boundary

```text
type/parameter_contract_carrier_missing
type/parameter_contract_row_drift
type/parameter_contract_duplicate_index
type/parameter_contract_implicit_receiver_forbidden
type/parameter_arity_mismatch
type/parameter_contract_violation
type/parameter_check_after_effects_forbidden
type/parameter_mir_type_as_proof_forbidden
type/parameter_value_fact_as_entry_proof_forbidden
type/caller_parameter_authority_forbidden
type/parameter_contract_elision_forbidden
type/backend_parameter_contract_capability_missing
type/backend_parameter_contract_silent_drop
type/mir_json_parameter_contract_missing
type/mir_json_parameter_contract_malformed
type/hakocli_by_name_parameter_authority_forbidden
```

Stable externally asserted tags should be defined once with the owning module;
do not duplicate string literals across VM, exporter, and backend code.

## Required Fixtures

```text
direct exact-numeric parameter success and wrong-type rejection
nested and recursive final-callee entry checks
method reroute validates only the final MIR callee
contracted missing/extra argument rejects before row indexing
unannotated legacy arity behavior remains unchanged
one contracted + one unannotated formal still requires full exact arity
implicit me produces no row
caller fact and MirType cannot elide the runtime check
missing/duplicate/drifted carrier rejects
MIR JSON exports the complete typed row
PyVM/LLVM/AOT/Wasm active-row modules reject before execution
HakoCli name/args helper is absent from the contract success path
```

## Non-Claims

```text
parameter_contract_activation = 0
parameter_entry_exact_numeric_contract_activation = pending
all_parameter_types_activated = 0
legacy_unannotated_exact_arity_activation = 0
implicit_receiver_contract_activation = 0
parameter_runtime_check_elision = 0
caller_side_parameter_authority = 0
extern_ffi_parameter_contract_activation = 0
closure_runtime_arg_contract_activation = 0
pyvm_parameter_contract_support = 0
llvm_aot_parameter_contract_support = 0
wasm_parameter_contract_support = 0
backend_parameter_abi_rewrite = 0
local_contract_activation = 0
return_contract_activation = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Acceptance

```text
function_entry_contract_owner_count = 1
parameter_entry_exact_numeric_contract_activation = 1
final_callee_contract_check = 1
contracted_function_exact_arity = 1
runtime_check_before_bind_and_body = 1
runtime_check_elision = 0
implicit_me_contract_rows = 0
mir_json_parameter_contract_carrier = 1
unsupported_backend_fail_fast_before_effect = 1
caller_parameter_authority = 0
hakocli_by_name_parameter_authority = 0
runtime_backend_fallback = 0
```

## Closeout

```text
FunctionMetadata.parameter_entry_contracts = implemented
FunctionEntryContractOwner = implemented
final-callee check after method reroute = implemented
contracted-function exact arity = implemented
runtime check before register binding/body = implemented
implicit receiver typed flag and exclusion = implemented
semantic-refresh carrier rebuild = implemented
MIR JSON typed carrier export/validation = implemented
non-VM backend preflight rejection = implemented
caller authority = absent
runtime-check elision = absent
HakoCli by-name contract path = absent
```

The existing method router retains its recursive final-callee execution shape.
The entry owner runs only after `pre_exec_reroute` declines to reroute, so the
selected MIR function validates its own carrier. Direct, nested, recursive,
and class-rerouted fixtures all use the same owner.

The full serial lib suite reports 3572 pass / 50 fail / 32 ignored. The 50
failures remain in pre-existing grammar compatibility, global-state, and MIR
expectation families; no parameter-entry test is among them. The quick gate
stops at the pre-existing `parser_control_box.hako` naming-charter token check
(`syntax-3 path`), which is outside this card and was not bypassed.

Changed production source files remain below 800 lines. Two pre-existing Rust
files above the cap were not modified.

## Verification

```text
cargo check -q --all-targets --features vm-reference
cargo test -q parameter_entry --lib
cargo test -q --features vm-reference parameter_contracts --lib
cargo test -q --features vm-reference backend::mir_interpreter::exec --lib
cargo test -q --features vm-reference backend::mir_interpreter::exec::exact_numeric_ops::tests --lib
cargo test -q runner::mir_json_emit::tests --lib
cargo test -q backend_capability --lib
cargo build --release --bin hakorune
cargo test --lib -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Advance to
`3483-LANGV1-TYPE-GUARANTEE-RETURN-EXIT-DESIGN-STOP-001`. Do not open return
checking from parameter metadata or reuse parameter-entry facts as return
proof.
