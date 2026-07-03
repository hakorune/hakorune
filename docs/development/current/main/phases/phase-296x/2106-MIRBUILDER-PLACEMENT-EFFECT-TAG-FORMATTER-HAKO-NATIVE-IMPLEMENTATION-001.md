# 2106 - MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the second narrow Rust-oracle
parity pilot owner: `placement_effect_tag_formatter`.

The implementation accepts scalar strings only:

```text
enum_family + variant -> text
```

## Hako Source

```text
lang/src/compiler/lib/placement_effect_tag_formatter.hako
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

## Excluded Surface

```text
refresh_function_placement_effect_routes
refresh_module_placement_effect_routes
PlacementEffectRoute summary
placement_effect route sorting
metadata.placement_effect_routes mutation
```

## Acceptance

```text
hako_source_present = 1
hako_source_line_count < 800
scalar_string_boundary = 1

source_selfhost_claim = 0
hako_adopted_decision = 0
placement_effect_route_collection_migration = 0
metadata_placement_effect_routes_mutation = 0
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  PlacementEffectTagFormatterHakoNativeImplementationAdded

selected_next_card:
  MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-PARITY-GATE-001
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
