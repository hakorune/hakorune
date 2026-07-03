# 2104 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-001

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-001
```

## Purpose

Select the second small hand-authored `.hako` native owner parity pilot after
the `storage_class_classifier` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selection Policy

```text
candidate_ranking_is_advisory = 1
manual_target_selection_allowed = 1
correctness_proof_is_parity_gate = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Selected Pilot

```text
selected_owner:
  placement_effect_tag_formatter

selected_rust_surface:
  src/mir/placement_effect.rs PlacementEffect enum/tag display text

selected_next_card:
  MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Included Surface

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

## Evidence

```text
selection_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hako-native-owner-parity-pilot-selection-rerun-001-v0.json

source_file:
  src/mir/placement_effect.rs
```

## Next Sequence

```text
1. MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001
   Dump stable JSON input/output pairs for PlacementEffect enum/tag text.

2. MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
   Hand-write the `.hako` implementation.

3. MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-PARITY-GATE-001
   Run `.hako` against oracle JSON and diff normalized output.

4. MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-ADOPTION-DECISION-001
   Allowed only after parity is green.
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no native seed materialization
no generated artifact edit authority
no runtime fallback
no new backend route
no new ABI
no metadata refresh migration
no placement_effect route collection migration
```
