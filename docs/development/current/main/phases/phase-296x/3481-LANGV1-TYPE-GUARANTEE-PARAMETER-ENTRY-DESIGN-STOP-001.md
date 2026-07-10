# 3481 - LANGV1-TYPE-GUARANTEE-PARAMETER-ENTRY-DESIGN-STOP-001

## Status

Active design consultation after the exact-numeric Box field first slice
closes in 3480.

Decision: required before implementation.

Implementation: stopped.

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

Candidate A is structurally preferred, but not accepted by this card.

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

Do not edit VM entry, MIR call lowering, MIR JSON, or backend parameter ABI
until this consultation accepts one owner, carrier, subset, fail-fast boundary,
and first implementation slice.
