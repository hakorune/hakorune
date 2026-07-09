# 3397 - MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001

## Token

```text
MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-ROUTE-DECISION-AUTHORITY-PILOT-001
```

## Purpose

Materialize the scoped `.hako` route-decision authority pilot for
CollectionScalarI64Routes.

This is the selected implementation card from 3396.

## Expected Scope

```text
surface:
  CollectionScalarI64Routes

authority source:
  COLLECTION_LEN_SCALAR_I64_HAKO_POLICIES

route rows:
  MapEntryCount -> MapLen, receiver_domain = MapBox
  ArraySlotLen -> ArrayLen, receiver_domain = ArrayBox
  StringLen -> StringLen, receiver_domain = StringBox
  AnyLength -> AnyLen, receiver_domain = Box

Rust role:
  oracle / compat checker retained

mismatch policy:
  fail-fast
```

## Implementation Requirements

```text
1. Read generated typed `.hako` policy row.
2. Build `.hako` route decision from the typed row.
3. Build Rust oracle decision from the existing Rust path.
4. Fail-fast compare route_kind, receiver_domain metadata, core_op,
   lowering_tier, return_shape, value_demand, publication_policy,
   effect_class, and proof_or_policy_source.
5. Consume the `.hako` decision only on match.
6. Do not add runtime fallback.
```

## Result

```text
collection_hako_route_decision_authority_pilot = 1
collection_hako_authority_result_consumed = 1
collection_rust_oracle_compat_checker = 1
collection_mismatch_fail_fast = 1
collection_live_route_calls_authority_pilot = 1
collection_mixed_receiver_domain_guarded = 1
collection_anylength_box_domain_guarded = 1
```

## Decision

```text
decision:
  SelectCollectionAuthorityPilotRerun

reason_token:
  CollectionHakoRouteDecisionAuthorityPilotMaterialized

selected_next_card:
  MIRBUILDER-SCALAR-KNOWN-FASTPATH-COLLECTION-HAKO-AUTHORITY-PILOT-RERUN-001
```

## Non-Claims

```text
collection_anylength_global_box_authority = 0
receiver_domain_authority_switch = 0
receiver_domain_widening_authority = 0
receiver_domain_projection = 0
any_length_wildcard_selector = 0
runtime_box_domain_fallback = 0
scalar_known_hako_runtime_route_authority = 0
source_selfhost_claim = 0
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_scalar_known_fastpath_collection_hako_route_decision_authority_pilot_guard.sh
```
