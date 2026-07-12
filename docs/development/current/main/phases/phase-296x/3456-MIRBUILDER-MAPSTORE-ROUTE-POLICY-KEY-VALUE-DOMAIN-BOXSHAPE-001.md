# 3456 - MIRBUILDER-MAPSTORE-ROUTE-POLICY-KEY-VALUE-DOMAIN-BOXSHAPE-001

## Status

Active by explicit user priority change on 2026-07-12. Language v1
conformance closeout remains parked; this card is the selected selfhost
resume task. No language-v1 behavior, scope, evidence, or acceptance claim is
changed by the reprioritization.

This card changes representation and validation structure only. It must not
add an accepted route or move authority.

## Problem

`MapStoreI64` names the key domain, not the stored-value domain:

```text
MapStoreI64: key_domain = I64, stored_value_domain = Any
MapStoreAny: key_domain = Any, stored_value_domain = Any
```

The old `value_boundary=ScalarI64` tuple merged independent semantic axes. Its
duplication across source, generator, generated Rust, validators, and fixtures
allowed the error to remain internally consistent.

## Structural Delta

1. Define one typed `RoutePolicyRow` SSOT for MapStore policy data with:
   `policy_row_id`, `route_kind`, `key_domain`, `stored_value_domain`,
   `result_shape`, `effect_class`, `mutation_class`, `publication_policy`, and
   `authority_kind`.
2. Keep row data owned by the hand-authored Hako source. Use one generator to
   emit the typed decision payload and caller projection.
3. Keep the Rust route matcher and Rust oracle independent.
4. Replace caller/shadow tuple comparisons with one shared validator entry.
5. Remove the ambiguous `value_boundary` field from this MapStore contract.

Expected touchpoints:

```text
lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako
lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako
tools/rust_lifecycle/generate_write_set_mapstore_*_hako_policy.py
src/mir/generic_method_route_plan/generated/write_set_mapstore_*_hako_policy.rs
src/mir/generic_method_route_plan/caller_orientation.rs
src/mir/generic_method_route_plan/scalar_known_hako_shadow.rs
src/mir/generic_method_route_plan/write_routes.rs
src/mir/generic_method_route_plan/tests/map_set_routes/
```

`write_routes.rs` is an oracle/test touchpoint, not a route-matching rewrite
target for this card.

## Required Tests

```text
I64 key + Any stored value -> MapStoreI64
Any key + the same stored value -> MapStoreAny
changing only the stored-value type does not change route kind
unknown, missing, extra, or drifting typed fields -> fail-fast
caller and shadow consumers use the same validator
```

Tests must vary key and stored-value domains independently. A fixture that only
copies the generated tuple is not an independent oracle.

## Authority Boundary

```text
route matching = Rust write_routes.rs
policy row edit source = hand-authored Hako
decision payload = Rust artifact generated from Hako
compatibility veto = independent Rust validator / oracle
mutation and backend = downstream Rust
caller orientation = policy-row contract acceptance or rejection only
```

## Acceptance

```text
mapstore_route_policy_typed_row_ssot = 1
mapstore_key_stored_value_domains_separated = 1
mapstore_shared_policy_validator = 1
mapstore_independent_axis_tests = 1
route_behavior_change = 0
route_selection_authority_switch = 0
caller_orientation_authority_pilot = 0
runtime_mutation_authority = 0
backend_lowering_authority = 0
publication_execution = 0
source_selfhost_claim = 0
```

## Next

After the typed row, shared validator, independent Rust oracle comparison, and
all required tests are green, resume 3454. After a green 3454 fixture-backed
rerun, enter 3455 and park caller orientation before the focused
Fact/Plan/Boundary inventory.
