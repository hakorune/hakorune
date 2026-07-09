# 3424 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-CONSUMER-DESIGN-STOP-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-CONSUMER-DESIGN-STOP-001
```

## Purpose

Decide whether and how a caller-facing consumer may be opened for the
Collection caller-orientation contract artifact from 3423.

This is a design stop. No runtime path, route-selection switch, MIR emission,
backend lowering, mutation, publication, or ScalarKnown-wide authority may be
implemented under this card.

## Existing Boundary

The contract artifact references exactly these existing Collection policy rows:

```text
collection_map_entry_count_scalar_i64_routes
collection_array_slot_len_scalar_i64_routes
collection_string_len_scalar_i64_routes
collection_any_length_scalar_i64_routes
```

Receiver domain and the explicit `AnyLength -> Box` boundary remain owned by
the existing Collection policy/oracle. Caller orientation is metadata-only and
must not become a route selector.

## Consultation Questions

1. Is a single read-only caller-facing consumer appropriate for Collection?
2. If yes, what contract metadata may it observe without selecting a route?
3. What exact fail-fast checks prove that receiver-domain semantics, runtime
   dispatch, backend lowering, mutation, and publication remain outside the
   caller contract?
4. Does the mixed receiver-domain surface require a narrower sub-surface, or
   can the four-row contract remain one metadata-only scope?

## Non-Claims

```text
caller_orientation_runtime_path = 0
caller_runtime_dispatch_authority = 0
route_selection_authority_switch = 0
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
