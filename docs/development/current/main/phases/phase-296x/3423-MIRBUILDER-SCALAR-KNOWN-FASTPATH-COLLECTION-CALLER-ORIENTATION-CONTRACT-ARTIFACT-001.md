# 3423 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-CONTRACT-ARTIFACT-001
```

## Purpose

Materialize the CollectionScalarI64Routes caller-orientation contract as a
hand-authored `.hako` table and a checked-in generated typed Rust artifact.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Selection Authority

The remaining read-only Collection family is already accepted as an explicit
four-row mixed receiver-domain authority pilot. The caller contract is allowed
to reference that closed row set without duplicating receiver-domain semantics.

```text
MapEntryCount  -> collection_map_entry_count_scalar_i64_routes
ArraySlotLen   -> collection_array_slot_len_scalar_i64_routes
StringLen      -> collection_string_len_scalar_i64_routes
AnyLength      -> collection_any_length_scalar_i64_routes
```

`AnyLength / Box` remains an explicit row reference only. It is not a wildcard
selector, global Box authority, receiver widening, or runtime fallback.

## Ownership

```text
caller-orientation source authority:
  lang/src/compiler/lib/collection_scalar_i64_caller_orientation_contract.hako

route semantics authority:
  lang/src/compiler/lib/collection_len_scalar_i64_policy_classifier.hako

generated typed artifact:
  src/mir/generic_method_route_plan/generated/
    collection_scalar_i64_caller_orientation_contract.rs
```

The caller contract carries only existing policy row IDs and metadata-only
flags. Receiver domain, route kind, core operation, and proof policy remain in
the existing Collection policy artifact.

## Required Delta

1. Add exactly four `.hako` contract rows for the existing policy row IDs.
2. Add a deterministic generator and typed Rust artifact.
3. Register the generated module without a live consumer.
4. Add a guard for exact row-set parity, artifact freshness, no consumer, and
   preserved AnyLength/Box explicit-row boundary.

## Non-Claims

```text
caller_orientation_runtime_path = 0
caller_runtime_dispatch_authority = 0
route_selection_authority_switch = 0
hako_runtime_route_authority = 0
scalar_known_hako_runtime_route_authority = 0
receiver_domain_authority_switch = 0
receiver_domain_widening_authority = 0
any_length_wildcard_selector = 0
runtime_box_domain_fallback = 0
backend_lowering_authority = 0
runtime_mutation_authority = 0
publication_execution = 0
collection_to_scalar_known_wide_authority = 0
source_selfhost_claim = 0
```

## Out Of Scope

- any live contract consumer
- Write or Delete caller orientation
- route selection, MIR emission, backend lowering, mutation, publication
- ScalarKnown-wide authority
