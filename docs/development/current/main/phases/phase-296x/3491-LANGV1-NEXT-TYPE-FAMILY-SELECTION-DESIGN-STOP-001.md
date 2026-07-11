# 3491 - LANGV1-NEXT-TYPE-FAMILY-SELECTION-DESIGN-STOP-001

## Status

Decision accepted. Candidate A, record construction/update, is the only next
type-contract family. Implementation remains queued behind the 3492 parameter
BindingId corrective and its green FULL grammar gate.

Decision: accepted.

## Objective

Select exactly one next Language v1 type-contract family after the
exact-numeric island: record construction/update or typed `Array<T>` element.
Fix its semantic owner, carrier, enforcement timing, backend capability, and
minimum implementation slice without treating `MirType` or storage plans as
contract authority.

## Candidate A - Record Construction/Update

Current implementation evidence:

```text
canonical grammar:
  record declaration
  record literal
  record with-update

focused builder owner:
  src/mir/builder/record_values.rs

current structural rejects:
  unknown record
  generic unsupported
  constructor arity mismatch
  duplicate/unknown update field
  unsupported escape/use shape

runtime evidence:
  src/tests/record_construction_ergonomics.rs

missing contract substrate:
  no typed RecordConstructionContract carrier
  no field-by-field runtime/proof disposition
  no dedicated backend capability row
```

Record is identity-free and has closed construction/update boundaries. This is
the recommended next family if the existing builder owner can project a typed
carrier without turning storage layout into semantic authority.

## Candidate B - Typed Array Element

Current implementation evidence:

```text
syntax/runtime surface:
  Array literal and ArrayBox mutation/read methods

current type evidence:
  src/mir/builder/types/array_element.rs
  receiver-local MirType::Array element publication
  typed-object collection element facts

backend/JSON evidence:
  generic Array routes and many storage/route plans

missing contract substrate:
  no source-owned typed Array<T> semantic carrier identified
  no single element-write contract owner
  MirType inference is representation evidence, not contract proof
  PackedArray<T> autouse remains representation migration debt
```

Typed Array must not be selected merely because route and storage machinery is
large. A source-owned contract and mutation boundary must be identified first.

## Decisions Required

1. Select A or B as the only next family. Recommended: A, record
   construction/update.
2. Name the single semantic owner and exact check timing.
3. Define the source-owned contract metadata and typed MIR carrier.
4. Define runtime-check versus verifier-proof dispositions. Representation
   facts cannot elide a check.
5. Define constructor/default/update evaluation order and exactly-once rules.
6. Define MIR JSON transport and central backend capability behavior.
7. Decide the VM first-slice subset and unsupported backend fail-fast.
8. Define stable tags, fixture matrix, claims, and explicit non-claims.

## Accepted Decision

```text
selected family: record construction + record with-update
single semantic owner: RecordValueContractOwner
boundaries: RecordConstruction | RecordWithUpdate
first consumer: Rust MIR interpreter / VM
unsupported consumers: PyVM / LLVM / AOT / Wasm reject before effects
typed Array<T>: unselected and inactive
```

The source-owned declaration inventory is the existing `RecordDecl` family.
The implementation may introduce domain wrappers for record and field
identity, but it must not invent a second declaration allocator or use a name,
`MirType`, storage layout, or backend route as proof. Stable identity is
projected from the declaration owner and validated by `semantic_refresh`.

The accepted function-owned `RecordValueContract` has one row per construct or
with-update publication. It carries declaration identity/schema fingerprint,
destination `ValueId`, optional update base, and declaration-ordered field
rows. Each field row carries declaration identity, final value, source
contract, and one explicit disposition:

```text
AnyDefault
RuntimeCheckedContract
VerifierProvenContract(fresh proof only)
UnsupportedFailFast
```

Representation facts cannot select a disposition or elide a runtime check.
The first slice defaults active field contracts to runtime checking.

### Evaluation and publication law

Construction performs structural/backend preflight first, evaluates explicit
field expressions exactly once in source order with immediate checks, then
evaluates missing defaults exactly once in declaration order with immediate
checks. It publishes declaration-ordered fields only after every check passes.

With-update performs structural/backend preflight, evaluates the base exactly
once, validates its record/schema before update-expression effects, checks
unchanged final fields in the first slice, evaluates update expressions exactly
once in source order with immediate checks, and publishes a replacement value
without mutating the base.

### Backend, tags, and fixtures

`semantic_refresh` rebuilds and validates the carrier before verifier, MIR
JSON, VM, backend preflight, or direct-tool consumption. MIR JSON is transport,
not authority. One central `RecordValueContracts` capability covers construct
and with-update. Unsupported consumers reject before effects without VM
fallback.

Stable tags are defined once under the selected owner:

```text
type/record_contract_unknown_record
type/record_contract_generic_unsupported
type/record_contract_constructor_arity_mismatch
type/record_contract_duplicate_field
type/record_contract_unknown_field
type/record_contract_missing_required_field
type/record_contract_default_unsupported
type/record_contract_field_contract_unsupported
type/record_contract_field_runtime_mismatch
type/record_contract_update_base_mismatch
type/record_contract_stale_carrier
type/record_contract_source_drift
type/record_contract_refresh_bypass
type/record_contract_representation_as_proof
type/record_contract_backend_unsupported
```

Fixtures cover source/declaration order, exactly-once evaluation, defaults,
replacement-not-mutation, updated/unchanged field checks, structural preflight,
wrong-base rejection before update effects, stale/drift/bypass, representation
as proof, MIR JSON, VM consumption, unsupported backends, and typed Array still
inactive.

## Accepted Next Card

`3493-LANGV1-RECORD-VALUE-CONTRACT-OWNER-001` is queued, not active, until
3492 closes green.

## Source Authority

```text
language law:
  docs/reference/language/semantic-contract-charter.md
  docs/reference/language/types.md

record semantics:
  record = identity-free value
  with = replacement producing a new value

type guarantee law:
  one owner per boundary
  runtime check or fresh verifier proof
  unsupported backend fails before effects

implementation evidence:
  record_values builder and record runtime fixtures
  array element fact owner and Array route/storage metadata
```

## Non-Authority

```text
MirType or value_types alone
record/array storage layout
route count or source count
green VM execution alone
PackedArray autouse plans
existing AST/MIR representation
backend helper availability
```

## Minimum Allowed Slice After Decision

One family, one owner, one typed carrier, one VM consumer, one backend
capability gate, and focused positive/negative fixtures. Do not combine record
and Array activation, FFI, proof elision widening, backend lowering, ownership
changes, or Failure/Outcome migration.

## Explicit Non-Claims

```text
next_type_family_selected = 1
selected_next_type_family = record_construction_update
record_contract_activation = 0
typed_array_contract_activation = 0
new_backend_contract_lowering = 0
runtime_check_elision_widened = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Consultation Stop

Closed by the accepted decision above. Do not edit record implementation code
until 3492 is green and 3493 becomes the current blocker.
