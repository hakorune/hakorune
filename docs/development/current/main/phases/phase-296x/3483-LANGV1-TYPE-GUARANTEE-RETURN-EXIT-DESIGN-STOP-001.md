# 3483 - LANGV1-TYPE-GUARANTEE-RETURN-EXIT-DESIGN-STOP-001

## Status

Active design consultation after 3482 closes the exact-numeric parameter-entry
contract.

Decision: required before implementation.

Implementation: stopped.

## Objective

Select one callee-exit exact-numeric contract owner and typed carrier without
duplicating checks in each Return instruction, caller result handling, MIR
JSON, and backend lowering.

## Current Inventory

```text
source / AST:
  declared return annotation is preserved

MIR metadata:
  declared_return_type_name is preserved
  exact_numeric_return_fact is advisory representation/fact metadata

MIR execution:
  each Return produces a VMValue
  exec_function_inner returns that value to direct/nested/recursive callers
  no general declared-return semantic check exists

MIR JSON / product backends:
  no accepted executable return-exit contract carrier
  no return-exit backend capability row
```

`FunctionSignature.return_type`, `MirType`, `declared_return_type_name`,
`exact_numeric_return_fact`, caller expectations, and successful VM execution
are non-authority for the runtime result's semantic truth.

## Consultation Questions

1. Confirm `FunctionReturnContractOwner` as the sole authoritative value-check
   owner. Should it validate the final callee result after cleanup but before
   publication to any caller?
2. Select the first subset: explicit exact-numeric return annotations only.
3. Define a function-owned typed carrier and its relationship to
   `exact_numeric_return_fact` and `FunctionSignature.return_type`.
4. Decide `return` without a value, implicit fallthrough, and `void` handling
   for functions with an active non-void contract.
5. Define multiple Return sites, cleanup outcome precedence, and rerouted final
   callee behavior under one exit owner.
6. Decide whether the first slice must check every runtime result
   unconditionally, with proof elision forbidden as at parameter entry.
7. Define MIR JSON transport and backend capability preflight. VM success must
   not authorize PyVM, LLVM/AOT, or Wasm.
8. Keep extern/FFI results and closure runtime invocation formally excluded or
   select a separate boundary owner.
9. Select the minimum substantive implementation slice and fixture matrix.

## Candidate

```text
A. final-callee exact-numeric return-exit contract
   - FunctionMetadata owns one typed return contract
   - Return sites produce pending values only
   - FunctionReturnContractOwner validates after final outcome/cleanup
   - validation occurs before caller publication
   - MIR JSON exports the same typed carrier
   - unsupported backends fail before program effects
   - no caller authority and no proof elision in the first slice

B. validate independently at every Return instruction
   - rejected unless it preserves one owner across cleanup/fallthrough

C. caller-side result checks
   - rejected because callers do not own callee guarantees

D. park return activation and move to locals
```

Candidate A is structurally preferred but not accepted by this card.

## Required Fail-Fast Boundary

```text
missing active return carrier does not mean pass
MirType or exact_numeric_return_fact alone is not proof
non-void contract cannot publish Void/fallthrough
contract failure occurs before caller-visible publication
cleanup/final outcome is checked exactly once
caller-side acceptance has no authority
unsupported backend rejects before program effects
VM success is not product-backend fallback
```

## Non-Claims

```text
return_contract_activation = 0
return_runtime_check = 0
return_proof_elision = 0
caller_side_return_authority = 0
mir_json_return_contract_carrier = 0
backend_return_contract_lowering = 0
extern_ffi_return_contract = 0
closure_runtime_return_contract = 0
local_contract_activation = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Stop Rule

Do not edit Return execution, cleanup outcome handling, call-result
publication, MIR JSON return metadata, or backend return ABI until this
consultation accepts one owner, carrier, subset, ordering, and fail-fast
boundary.
