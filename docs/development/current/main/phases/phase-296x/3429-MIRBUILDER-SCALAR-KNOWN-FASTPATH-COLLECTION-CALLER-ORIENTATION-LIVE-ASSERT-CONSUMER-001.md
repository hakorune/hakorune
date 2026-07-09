# 3429 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-CALLER-ORIENTATION-LIVE-ASSERT-CONSUMER-001
```

## Purpose

Extend the compiler-side assertion-only caller consumer to the exact four
Collection read policy rows. The mixed receiver domains and explicit
`AnyLength -> Box` rule remain owned by the existing policy/oracle.

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Required Delta

1. Add a Collection row-ID assertion over the generated four-row contract.
2. Call it after the existing Collection policy lookup.
3. Pass only `policy_row_id: &str`; do not pass receiver domain, route kind,
   core operation, or route decision.
4. Add exact four-row, unknown-ID, metadata-drift, and live-call guards while
   retaining the existing `AnyLength -> Box` oracle guard.

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
new_backend_route = 0
new_abi = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/rust_lifecycle_mirbuilder_scalar_known_fastpath_collection_caller_orientation_live_assert_consumer_guard.sh
```

## Result

```text
status = landed
collection_caller_orientation_live_assert_consumer = 1
collection_four_row_exact = 1
anylength_box_boundary_retained = 1
assertion_only = 1
tests = 9 passed
receiver_domain_authority_switch = 0
caller_orientation_runtime_path = 0
source_selfhost_claim = 0
selected_next_card =
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-READ-CALLER-ORIENTATION-ASSERTION-CLOSEOUT-001
```
