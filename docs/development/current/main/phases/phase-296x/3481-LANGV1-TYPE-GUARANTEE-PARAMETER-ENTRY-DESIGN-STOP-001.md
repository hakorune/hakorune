# 3481 - LANGV1-TYPE-GUARANTEE-PARAMETER-ENTRY-DESIGN-STOP-001

## Status

Active design consultation after the exact-numeric Box field first slice
closes in 3480.

Decision: accepted.

Implementation: delegated to 3482.

## Accepted Decision

```text
owner = FunctionEntryContractOwner
check_target = final callee
order = final callee resolution -> exact arity -> contract -> bind -> body
first_subset = explicit exact-numeric source parameters
carrier = FunctionMetadata.parameter_entry_contracts[]
implicit_me = excluded
runtime_check_elision = forbidden
first_supported_backend = MIR interpreter VM only
extern_ffi = excluded
closure_runtime_arguments = excluded by the existing strict gate
caller_authority = forbidden
```

Functions with at least one active parameter-entry contract require exact
arity across their complete formal parameter list. Unannotated functions keep
their current legacy arity behavior during this narrow migration slice.

`FunctionSignature.params`, `MirType`, `declared_param_decls`, caller facts,
and `exact_numeric_value_facts` are not proof that an incoming runtime value
satisfies the contract. `MirType` may remain a derived representation fact,
but its projection must be structurally separated from semantic contract-row
construction and the stale metadata comment must be corrected.

The current method router may retain its recursive final-callee execution
shape. The authoritative check runs only after `pre_exec_reroute` declines to
reroute, so a rerouted MIR function reaches its own entry owner before binding
or body effects. Do not check the original non-executed function metadata.

## Accepted Carrier

```text
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

Function identity and freshness come from the owning `MirFunction` and its
normal semantic-refresh lifecycle. Do not copy a function name or fabricate an
epoch into every row unless implementation evidence proves that the owner
boundary cannot validate freshness without it.

## Accepted Backend Boundary

The first implementation supports the Rust MIR interpreter only. PyVM, LLVM,
AOT, and Wasm reject modules carrying active parameter-entry contracts through
the central MIR backend capability gate until each has a typed carrier
consumer. VM success is not fallback authority.

The LLVM Python `_seed_hakocli_args_array_fact` helper is migration debt and a
future retirement target. It is neither a contract source nor an allowed
consumer for this slice, and no new parameter behavior may be added to it.

## Objective

Select one executable parameter-entry contract carrier without duplicating
checks at callers, VM entry, MIR JSON, and backend ABI boundaries.

## Current Inventory

```text
parser / AST:
  ParamDecl preserves name + declared_type_name

MIR builder:
  FunctionMetadata.declared_param_decls preserves source annotation
  FunctionSignature.params / MirType is a callable representation fact

exact numeric facts:
  declared_param_decls can seed exact_numeric_value_facts
  those facts do not check incoming runtime values

VM entry:
  execute_function_with_args binds params and args directly
  no general declared-parameter contract check

MIR JSON / EXE-AOT:
  function params carry id/name/MirType representation
  declared_param_decls is not a live semantic contract carrier

backend preflight:
  no parameter-entry contract capability row
```

Therefore `MirType::Integer`, exact numeric param facts, parser acceptance, and
caller-side argument shape are non-authority for parameter semantic truth.

## Consultation Questions

1. Confirm `FunctionEntryContractOwner` as the only value-check owner. Should
   callers carry proof hints only, never perform the authoritative check?
2. Select the first parameter subset: exact numeric annotations only, or a
   different closed type family.
3. Define the MIR contract carrier keyed by function + parameter index/value +
   declared type, including instance-method receiver handling.
4. Decide whether implicit `me` is always excluded unless it has a future
   explicit source contract.
5. Define runtime-check elision. Is caller proof consumable at callee entry, or
   should the first slice retain checks unconditionally?
6. Define MIR JSON transport and backend capability preflight. Which backends
   may enforce the first slice, and where must unsupported targets fail?
7. Define direct VM calls, nested MIR calls, external calls, closures, and
   recursive calls under the same entry owner.
8. Select the minimum substantive implementation slice and fixture matrix.

## Candidate

```text
A. callee-entry exact-numeric contract
   - FunctionMetadata owns typed contract rows
   - VM checks after arity validation and before body execution
   - MIR JSON exports the same rows
   - backend capability preflight rejects unsupported targets
   - implicit me excluded
   - no caller-side authority
   - no runtime-check elision in the first slice

B. caller-side argument checks
   - rejected unless a single-owner proof shows callee entry need not recheck

C. park parameter activation and move to return exit
```

Consultation accepted Candidate A. The executable task is
`3482-LANGV1-TYPE-GUARANTEE-PARAMETER-ENTRY-EXACT-NUMERIC-CONTRACT-001`.

## Required Fail-Fast Boundary

```text
missing contract carrier does not mean pass
MirType/declared metadata alone does not prove runtime value
arity is checked before contract indexing
contract failure occurs before callee body effects
unsupported backend rejects before program effects
VM success is not EXE/AOT fallback
implicit me is not silently treated as a user annotation
```

## Source Authority

```text
3479 accepted type-guarantee decision
3480 guarantee matrix and structural proof boundary
AST ParamDecl
FunctionMetadata.declared_param_decls
VM execute_function_with_args entry
MIR JSON function export
central MIR backend capability gate
```

## Non-Claims

```text
parameter_contract_activation = 0
parameter_runtime_check = 0
parameter_proof_elision = 0
caller_side_parameter_authority = 0
mir_json_parameter_contract_carrier = 0
backend_parameter_contract_lowering = 0
implicit_me_contract = 0
local_contract_activation = 0
return_contract_activation = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Stop Rule

Satisfied by the accepted decision above. Implementation may proceed only
inside 3482's exact-numeric entry scope; return/local/FFI/closure-runtime-arg
contracts and backend parameter ABI lowering remain stopped.
