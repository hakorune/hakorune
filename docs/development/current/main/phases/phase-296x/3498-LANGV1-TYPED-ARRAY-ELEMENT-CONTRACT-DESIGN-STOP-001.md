# 3498 - LANGV1-TYPED-ARRAY-ELEMENT-CONTRACT-DESIGN-STOP-001

## Status

Active design consultation stop. Do not change source acceptance, MIR
contracts, runtime checks, or backend capability before this decision closes.

## Trigger

3497 closed one behavior and identity owner for every accepted Array write:

```text
ArrayElementWriteOwner
ArrayElementWrite { site_id, kind, receiver, index, value }
ArrayStateTerm / opaque runtime ArrayStateIdentity
LiteralAppend | Push | Set | Insert
```

The guarantee matrix still lists typed `Array<T>` as a target contract, but
no source-owned element carrier or state-attached contract law is active.
`MirType::Array`, homogeneous literal inference, storage specialization, and
planner routes remain representation evidence only.

## Decision Questions

1. Which explicit source sites create a typed element contract: `Array<T>`
   annotations only, constructor type arguments, or both?
2. Does the contract attach to `ArrayStateIdentity`, preserving one contract
   through aliases/calls/fields/returns/PHI/share while deep clone creates a
   fresh state carrying the same spec?
3. Is the first subset exact-numeric `Array<T>` only, and which spellings?
4. What source spec and refreshed carrier identify state without exposing a
   runtime ID, Arc pointer, Box ID, storage variant, ValueId, or `MirType`?
5. For every write kind, what is the exact evaluate/check/mutate/result order?
   A failed check must not mutate the Array.
6. Does `Any -> T` always runtime-check in the first slice, with proof elision
   forbidden and representation facts rejected as proof?
7. How are conflicting claimed contracts for one aliased state rejected?
8. Which one VM consumer is supported, and which backends fail before effects?
9. Are construction append and later mutation one state carrier or separate
   carriers under one owner?
10. Which stale/missing/drift/bypass cases and stable tags close the boundary?

## Authority / Non-Authority

Authority is explicit source `TypeContractSpec`, canonical
`ArrayElementWrite`, Array state identity law, semantic-refresh carrier, one
element-contract owner, runtime value checker, and backend preflight.

Non-authority is `MirType::Array`, homogeneous inference, storage/layout,
planner/helper names, ValueId/BindingId/Box ID/pointers, source names, test
counts, or successful VM execution.

## Candidate First Slice

```text
LANGV1-TYPED-ARRAY-EXACT-NUMERIC-ELEMENT-CONTRACT-001

owner = TypedArrayElementContractOwner
source = explicit exact-numeric Array<T>
write vocabulary = ArrayElementWrite
carrier = state-attached and semantic_refresh-owned
consumer = one VM runtime checker
unsupported backend = pre-effect fail-fast
runtime check elision = 0
```

## Required Non-Claims

```text
typed_array_contract_activation = 0
source_owned_array_element_contract = 0
runtime_array_element_type_check = 0
ordinary_collection_contract_activation = 0
homogeneous_literal_inference_authority = 0
runtime_check_elision_widened = 0
backend_array_lowering = 0
broad_static_type_checker = 0
runtime_backend_fallback = 0
selfhost_claim = 0
```

## Stop Rule

Stop with a clean tree after publishing the consultation packet. Create an
implementation card only after owner, state law, carrier, check order,
backend support, tags, fixtures, and non-claims are accepted together.
