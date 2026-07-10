# 3484 - LANGV1-TYPE-GUARANTEE-RETURN-EXIT-EXACT-NUMERIC-CONTRACT-001

## Status

Complete. The final-callee return owner, typed carrier, Void/fallthrough veto,
MIR JSON transport, and backend boundary are implemented and verified.

Decision: accepted by 3483.

Implementation: complete.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Objective

Activate explicit exact-numeric return annotations at one final-callee exit.
Validate the final runtime result exactly once after cleanup CFG execution and
before caller publication, reject missing outcomes structurally, transport the
same carrier through MIR JSON, and fail unsupported backends before effects.

## Structural Owners

```text
ExactNumericRuntimeValueChecker:
  shared subordinate value/type/range checker

FunctionEntryContractOwner:
  incoming argument timing and carrier owner; unchanged

FunctionReturnContractOwner:
  final outgoing result timing and carrier owner

ReturnOutcomeVerifier:
  active non-void contract reachable-outcome existence veto
```

Do not put return policy into the shared numeric checker. It receives a value
and declared type only; entry/exit owners supply timing, carrier, and tags.

## Typed Carrier

Add one function-owned typed carrier:

```text
FunctionMetadata.return_exit_contract: Option<ReturnExitContract>

ReturnExitContract {
  contract_id
  declared_type_name
  contract_kind = ExactNumeric
  void_policy = RejectVoid
  runtime_check_required = true
  proof_elision_allowed = false
  backend_capability_required = return_exit_exact_numeric
  source_return_annotation_present = true
  owner = FunctionReturnContractOwner
}
```

Use typed enums internally for kind, Void policy, and owner. JSON may project
stable string names. Rebuild the carrier from `declared_return_type_name`
during semantic refresh and validate exact equality. No Return ValueId list,
function-name policy, source-path inference, or persisted cache is allowed.

## Ordered Implementation Tasks

### 1. Shared exact-numeric value checker

- extract parameter-entry's VMValue type/range logic into one subordinate
  exact-numeric checker;
- preserve parameter-entry behavior and diagnostics;
- accept dynamic Integer only when it fits the declared exact type;
- accept ExactNumeric only under the accepted declared-source-type rule;
- return structured failure so entry and exit owners attach boundary tags;
- do not merge entry and return carrier or timing owners.

### 2. Return carrier and semantic refresh

- define typed `ReturnExitContract` and add it to `FunctionMetadata`;
- build it only for explicit exact-numeric return annotations;
- keep `FunctionSignature.return_type` as representation projection;
- keep `exact_numeric_return_fact` advisory and non-authoritative;
- fail on missing, extra, malformed, or declaration-drifted carriers;
- update the guarantee matrix only for this scoped exact-numeric activation.

### 3. ReturnOutcomeVerifier

- add a focused verifier under the existing MIR verification layer;
- walk reachable CFG from the function entry using terminator edges;
- for active non-void carriers, reject reachable `Return(None)`;
- reject reachable unterminated/fallthrough blocks;
- do not require every syntactic block to return when it is unreachable;
- do not introduce general static type checking or reinterpret cleanup;
- use the same verifier from normal compile verification and VM preflight for
  manually constructed MIR fixtures.

### 4. Final-callee VM exit owner

- invoke `FunctionReturnContractOwner` only in the central
  `BlockOutcome::Return(result)` branch;
- check after cleanup CFG has produced its final Return;
- check before restoring/publishing the value to the caller;
- preserve frame/call-depth restoration on both success and contract error;
- reject `VMValue::Void` under active exact-numeric contracts;
- let Fault/VM error bypass return-value validation and remain primary;
- do not add checks at individual Return instructions or caller `write_result`.

### 5. MIR JSON transport

- export declared return evidence and `metadata.return_exit_contract`;
- validate carrier consistency before export;
- add accepted, missing, malformed, and drifted carrier fixtures;
- do not export Return operand ValueIds as function contract truth;
- do not let consumers infer contracts from `MirType` or result instructions.

### 6. Central backend capability

- add `return_exit_exact_numeric` inspection to the central MIR backend gate;
- support only the Rust MIR interpreter in this slice;
- reject PyVM, LLVM/AOT, and Wasm before effects when a carrier exists;
- do not use VM fallback;
- do not use LLVM zero/null completion or type adjustment as contract support;
- leave backend ABI/lowering unchanged.

### 7. Focused fixture matrix

Required runtime fixtures:

```text
valid Integer and matching ExactNumeric return
wrong runtime type and out-of-range return
Return(None), explicit Void, and Void-valued Return rejection
two valid branch returns
one valid and one invalid branch return
reachable fallthrough/unterminated rejection
unreachable unterminated block ignored
direct, nested, recursive, ignored-result, and rerouted calls
body return through normal cleanup CFG
cleanup final Return overrides body value and is checked
cleanup Fault/VM error wins without return check
unannotated return behavior unchanged
```

Required carrier/backend fixtures:

```text
semantic-refresh rebuild and annotation drift
missing/extra/malformed carrier rejection
MirType and exact_numeric_return_fact cannot elide checks
MIR JSON typed carrier export and validation
PyVM/LLVM/AOT/Wasm preflight rejection
no caller authority
no LLVM zero/type-adjustment success path
extern/FFI and closure runtime invocation remain excluded
```

## Stable Fail-Fast Tags

```text
type/return_contract_carrier_missing
type/return_contract_carrier_drift
type/return_contract_void_forbidden
type/return_contract_violation
type/return_contract_fallthrough_forbidden
type/return_contract_check_after_publication_forbidden
type/return_mir_type_as_proof_forbidden
type/return_value_fact_as_proof_forbidden
type/caller_return_authority_forbidden
type/return_contract_elision_forbidden
type/mir_json_return_contract_missing
type/mir_json_return_contract_malformed
type/backend_return_contract_capability_missing
type/backend_return_contract_silent_drop
type/llvm_return_zero_fallback_as_contract_forbidden
type/backend_return_adjustment_as_contract_forbidden
```

Define stable strings once in their owning module. Tests may assert exported
constants; VM, exporter, verifier, and backend must not duplicate tag literals.

## Non-Claims

```text
return_contract_activation = 0
return_exit_exact_numeric_contract_activation = pending
all_return_types_activated = 0
return_proof_elision = 0
caller_side_return_authority = 0
extern_ffi_return_contract_activation = 0
closure_runtime_invocation_return_contract_activation = 0
pyvm_return_contract_support = 0
llvm_aot_return_contract_support = 0
wasm_return_contract_support = 0
backend_return_abi_rewrite = 0
local_contract_activation = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Acceptance

```text
function_return_contract_owner_count = 1
return_outcome_verifier_count = 1
return_exit_exact_numeric_contract_activation = 1
final_blockoutcome_return_check = 1
return_check_before_caller_publication = 1
cleanup_cfg_final_return_checked = 1
fault_error_precedence_retained = 1
void_return_rejected = 1
reachable_fallthrough_rejected = 1
runtime_check_elision = 0
mir_json_return_contract_carrier = 1
unsupported_backend_fail_fast_before_effect = 1
caller_return_authority = 0
llvm_zero_fallback_return_authority = 0
runtime_backend_fallback = 0
changed_production_source_over_800_lines = 0
```

## Verification

```text
focused carrier/semantic-refresh unit tests
ReturnOutcomeVerifier unit tests
VM direct/nested/recursive/reroute/cleanup tests with vm-reference
MIR JSON return-carrier tests
central backend-capability tests
cargo check -q --all-targets --features vm-reference
cargo build --release --bin hakorune
current-state pointer guard
git diff --check
```

## Next Queue

After this card closes, do not immediately activate locals. Open one local
exact-numeric design stop that inventories initialization, reassignment, PHI,
loop-carried values, Any refinement, and proof invalidation. The durable queue
is recorded in the Language v1 workstream, not as pre-created numbered cards.

## Closeout Evidence

```text
FunctionMetadata.return_exit_contract = implemented
FunctionReturnContractOwner = central BlockOutcome::Return branch
ReturnOutcomeVerifier = reachable CFG, Return(None), fallthrough
shared exact-numeric checker = parameter + return subordinate owner
MIR JSON typed carrier = implemented
backend capability = VM only; PyVM/LLVM/AOT/Wasm reject
runtime check elision = 0
caller authority = 0
```

Focused verification:

```text
type-contract matrix/carrier tests: 11 green
parameter-entry regression tests: 8 green
return VM direct/nested/recursive/reroute/cleanup tests: 9 green
return outcome verifier tests: 2 green
MIR JSON return carrier tests: 2 green
return backend capability tests: 2 green
shared backend capability tests: 2 green
cargo check --all-targets --features vm-reference: green
cargo build --release --bin hakorune: green
changed production source over 800 lines: 0
```

Next: 3485 local exact-numeric contract design stop.
