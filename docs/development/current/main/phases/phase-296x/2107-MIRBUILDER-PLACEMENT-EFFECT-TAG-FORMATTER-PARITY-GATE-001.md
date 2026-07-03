# 2107 - MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-PARITY-GATE-001
```

## Purpose

Add the executable parity gate for the second narrow Rust-oracle parity pilot:
`placement_effect_tag_formatter`.

The gate generates a temporary `.hako` EXE app from the Rust-oracle fixture and
diffs normalized output against expected enum/tag text.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-placement-effect-tag-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/placement_effect_tag_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_placement_effect_tag_formatter_parity_gate.sh
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_placement_effect_tag_formatter_parity_gate.sh
```

Expected output contract:

```text
output_contract=rust-lifecycle-mirbuilder-placement-effect-tag-formatter-parity-gate-v0
owner=placement_effect_tag_formatter
parity_rows=25
parity_status=green

source_selfhost_claim = 0
hako_adopted_decision = 0
metadata_refresh_migration = 0
placement_effect_route_collection_migration = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  PlacementEffectTagFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no route collection migration
no metadata refresh migration
no runtime fallback
no new backend route
no new ABI
```
