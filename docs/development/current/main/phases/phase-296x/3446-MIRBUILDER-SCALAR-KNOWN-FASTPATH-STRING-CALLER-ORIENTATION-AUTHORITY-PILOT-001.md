# 3446 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-STRING-CALLER-ORIENTATION-AUTHORITY-PILOT-001

## Status

Ready implementation task. Authority claims remain zero until the required
delta and guard are green.

## Exact Scope

```text
surface = StringScalarI64Routes
policy_row_ids = {
  string_indexof_scalar_i64_routes,
  string_lastindexof_scalar_i64_routes,
  string_contains_scalar_i64_routes
}
authority_scope = policy_row_id_contract_only
consumer_input = PolicyRowIdOnly
consumer_return = Unit
```

## Required Delta

1. Add a String authority validator beside the existing assertion-only caller
   consumer; resolve only the exact generated three-row contract set.
2. Compare each generated caller contract with the corresponding generated
   String route-decision policy metadata.
3. Retain the existing String `.hako` route decision and Rust oracle as the
   route authority and fail-fast veto; neither may become a fallback.
4. Test all three rows plus unknown, wrong-surface, missing/extra row, metadata
   drift, policy drift, and no runtime/backend/mutation/publication leakage.
5. Record a deterministic pilot fixture and guard, then schedule one freshness
   rerun followed by a design stop before Collection or Write.

## Stop Conditions

Stop instead of widening if the implementation needs route/value/effect input,
non-Unit output, hand-copied route semantics, MIR or ValueId emission, runtime
or backend consumption, mutation/publication, fallback, Collection, Write,
Delete, ScalarKnown-wide, or Source Selfhost scope.

## Allowed Completion Claims

```text
string_caller_orientation_authority_pilot = 1
string_caller_orientation_authority_scope = policy_row_id_contract_only
string_caller_orientation_consumer_unit_only = 1
string_exact_three_row_scope = 1
string_hako_route_decision_authority_retained = 1
string_rust_oracle_compat_checker_retained = 1
string_mismatch_fail_fast = 1
no_new_route_authority = 1
```

All runtime, backend, mutation, publication, Delete, wide, fallback, and Source
Selfhost claims remain zero.

## Implementation Result

The live caller validator now accepts only `policy_row_id`, resolves the exact
generated three-row String contract set, checks the generated policy metadata,
and returns `Unit`. The existing String `.hako` route decision and Rust oracle
compatibility veto remain in `scalar_known_hako_shadow.rs`.

```text
string_caller_orientation_authority_pilot = 1
string_caller_orientation_authority_scope = policy_row_id_contract_only
string_caller_orientation_consumer_unit_only = 1
string_exact_three_row_scope = 1
string_hako_route_decision_authority_retained = 1
string_rust_oracle_compat_checker_retained = 1
string_mismatch_fail_fast = 1
no_new_route_authority = 1
```

The implementation has no route selector, ValueId/MIR emission, runtime or
backend consumer, mutation/publication path, fallback, Delete, wide, or Source
Selfhost authority.
