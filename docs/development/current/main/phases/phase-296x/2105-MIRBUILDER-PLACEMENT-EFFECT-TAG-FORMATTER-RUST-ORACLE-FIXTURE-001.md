# 2105 - MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle JSON fixture for the second narrow `.hako` parity pilot:
`placement_effect_tag_formatter`.

This fixture fixes only the pure enum/tag formatting surface. It does not move
placement-effect route collection, route summary formatting, route sorting, or
metadata mutation into `.hako`.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-placement-effect-tag-formatter-rust-oracle-v0.json
```

## Covered Surface

```text
PlacementEffectSource -> text
PlacementEffectDecision -> text
PlacementEffectState -> text
PlacementEffectDemand -> text
PlacementEffectPublicationBoundary -> text
PlacementEffectBorrowContract -> text
```

## Acceptance

```text
oracle_row_count = 25
enum_family_count = 6
json_scalar_boundary = 1

source_selfhost_claim = 0
hako_adopted_decision = 0
placement_effect_route_collection_migration = 0
metadata_placement_effect_routes_mutation = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  PlacementEffectTagFormatterRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no route collection migration
no metadata refresh migration
no runtime fallback
no new backend route
no new ABI
```
