# 3494 - LANGV1-POST-RECORD-TYPE-FAMILY-SELECTION-DESIGN-STOP-001

## Status

Active design consultation stop. Do not change parser, MIR, VM, backend, or
runtime behavior until one remaining family has a source-owned contract and a
single enforcement owner.

Decision: pending.

## Objective

Select at most one next semantic type-contract family after the landed Box
field, parameter-entry, return-exit, local-slot, and record-value slices.
Selection must come from current source evidence, not from `MirType`, storage
layout, backend routes, helper names, or existing green execution.

## Required Inventory

For every remaining candidate, record:

```text
source-owned contract declaration
stable semantic identity
all construction/write/publication boundaries
candidate single enforcement owner
semantic_refresh rebuild inputs
VM consumer boundary
unsupported-backend pre-effect gate
MIR JSON transport need
current bypasses and representation-only consumers
```

At minimum inspect typed `Array<T>`, static tables, ordinary collections, Weak
fields, FFI ingress/egress, and remaining guarantee-matrix families.

## Consultation Questions

1. Which candidate already has exactly one source-owned semantic contract,
   stable identity, and one complete write/publication boundary?
2. For typed `Array<T>`, what owns the source element contract, and which one
   operation owns initialization plus mutation without using `MirType::Array`
   or storage inference as proof?
3. Are static tables and ordinary collections separate semantic families, or
   projections of the same element-contract owner?
4. Is the current Weak-field path an active complete contract, or a narrow
   representation/runtime check requiring its own closeout audit?
5. Should FFI remain deferred until all in-language value boundaries close?
6. Which candidate supports one VM consumer and central non-VM fail-fast
   without opening backend lowering or a broad static checker?
7. Does a candidate require an absence, optional state, truthiness, equality,
   or ownership decision first?

## Authority

```text
docs/reference/language/types.md
typed guarantee matrix and implementation status ledger
source declaration metadata
semantic_refresh carrier owners
current MIR write/publication operations
central backend capability manifest
```

## Non-Authority

```text
MirType or value_types alone
storage/layout metadata
route availability or helper names
source path or use count
green VM behavior
backend fallback or environment-selected behavior
```

## Fail-Fast Boundary

```text
no source-owned contract -> candidate cannot be selected
multiple write owners -> consolidate owner before activation
representation fact used as proof -> reject
missing semantic_refresh rebuild path -> reject
unsupported backend without pre-effect gate -> reject
selection requiring fallback or environment activation -> reject
```

## Minimum Next Slice

The accepted consultation must name exactly one family, one owner, one typed
carrier, one VM consumer, and one backend capability. The implementation card
must not activate two families together.

## Non-Claims

```text
next_type_family_selected = 0
typed_array_contract_activation = 0
static_table_contract_activation = 0
ordinary_collection_contract_activation = 0
ffi_contract_activation = 0
new_backend_contract_lowering = 0
runtime_check_elision_widened = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```
