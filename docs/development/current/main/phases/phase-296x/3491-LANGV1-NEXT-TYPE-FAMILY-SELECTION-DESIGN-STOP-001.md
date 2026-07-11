# 3491 - LANGV1-NEXT-TYPE-FAMILY-SELECTION-DESIGN-STOP-001

## Status

Active design consultation stop. Do not activate record or typed-Array
contracts until this decision is accepted.

Decision: pending.

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
next_type_family_selected = 0
record_contract_activation = 0
typed_array_contract_activation = 0
new_backend_contract_lowering = 0
runtime_check_elision_widened = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Consultation Stop

Return the selected candidate, owner, carrier schema, evaluation/check order,
backend policy, fixture matrix, stable tags, minimum implementation card, and
non-claims. Do not edit implementation code before acceptance.
