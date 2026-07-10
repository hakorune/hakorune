# 3480 - LANGV1-TYPE-GUARANTEE-BOX-FIELD-EXACT-NUMERIC-CONTRACT-001

## Status

Active substantive implementation card after 3479 accepts the type-guarantee
matrix vocabulary and selects the exact-numeric Box field boundary.

Decision: accepted by 3479.

Implementation: in progress.

## Objective

Materialize the closed annotation-site guarantee matrix and adopt the existing
exact-numeric Box field write path as the first Language v1 semantic-contract
owner without widening type checking.

## Structural Owner Map

```text
typed guarantee matrix:
  src/mir/type_contracts/guarantee_matrix.rs

exact-numeric Box field contract decision:
  src/mir/exact_numeric_field_contracts.rs

contract consistency veto:
  src/mir/verification/numeric_substrate.rs

runtime enforcement:
  src/backend/mir_interpreter/exec/numeric_contracts.rs

backend preservation/preflight:
  src/mir/exact_numeric_backend_capability.rs
  src/mir/backend_capability.rs
```

Do not create another field-write classifier or backend support table.

## Required Matrix

The typed artifact closes these sites:

```text
local initialization/reassignment
parameter entry
return exit
ordinary Box field initialization/write
record construction/with-update
static table element
ordinary collection element
typed Array<T> element
Weak field
FFI ingress/egress
backend preservation boundary
```

Every row records current guarantee class, target guarantee class, single
owner, activation state, and unsupported-backend policy. Non-active rows are
`metadata_only_non_guarantee` or an already-narrow checked contract; they are
not promoted by this card.

## Exact-Numeric Field Decision

For each exact-numeric `FieldSet`, the existing owner must produce exactly one
structural disposition:

```text
verifier_proven_contract:
  statically known value fits the declared exact range, or the dynamic Integer
  lane is wholly contained by the declared type

runtime_checked_contract:
  dynamic value needs a range/type check and a matching runtime-check contract
  is attached

semantic_reject:
  statically known value violates the declared range
```

Runtime-check elision is allowed only for a freshly re-derived
`verifier_proven_contract` disposition. Missing or mismatched disposition must
not silently pass the verifier.

## Freshness Contract

```text
semantic refresh clears/rebuilds exact-numeric field dispositions
site key = function + block + instruction index + field + value + declared type
declaration or instruction drift changes the key/re-derived result
no persisted cross-refresh cache
no fabricated CFG/SSA/verifier epoch
```

## Backend Boundary

If a selected backend cannot preserve an attached runtime check, exact storage,
or exact operation route fact, existing centralized backend preflight rejects
before execution. VM success is not fallback authority.

## Implementation Scope

```text
1. add typed 11-site guarantee matrix
2. add exact-numeric Box field disposition/proof vocabulary
3. rebuild dispositions with runtime-check contracts during semantic refresh
4. verify one disposition per active exact-numeric FieldSet
5. retain VM check-before-store behavior
6. retain centralized backend preflight
7. add focused positive/negative/freshness tests
8. document matrix and owner boundary
```

## Non-Claims

```text
all_colon_annotations_activated = 0
local_contract_activation = 0
parameter_contract_activation = 0
return_contract_activation = 0
all_box_fields_activated = 0
record_contract_change = 0
array_contract_change = 0
ordinary_collection_contract_activation = 0
ffi_contract_activation = 0
runtime_check_elision_global = 0
backend_contract_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Acceptance

```text
guarantee_matrix_site_count = 11
permanent_metadata_only_annotation = 0
box_field_exact_numeric_contract_activation = 1
box_field_contract_owner_count = 1
runtime_check_before_store = 1
runtime_check_elision_without_proof = 0
unsupported_backend_fail_fast = 1
vm_fallback_for_exe_aot = 0
source_files_over_800_lines = 0
```

## Verification

```text
focused type-contract matrix unit tests
exact_numeric_field_contracts unit tests
MIR numeric verifier tests
VM numeric runtime-contract tests
exact numeric backend-capability tests
current-state pointer guard
git diff --check
```
