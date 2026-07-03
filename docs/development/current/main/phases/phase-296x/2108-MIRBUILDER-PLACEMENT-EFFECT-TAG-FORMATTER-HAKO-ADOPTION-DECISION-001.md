# 2108 - MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-PLACEMENT-EFFECT-TAG-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `placement_effect_tag_formatter` as the second narrow Rust-oracle parity
pilot owner after a green 25-row `.hako` EXE parity gate.

This decision adopts only the pure tag formatter:

```text
PlacementEffect enum-family + variant -> text
```

It does not adopt placement-effect route collection, route summary formatting,
metadata refresh, function/module traversal, Source Selfhost, or full MirBuilder
conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-placement-effect-tag-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/placement_effect_tag_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_placement_effect_tag_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-placement-effect-tag-formatter-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 25
decision = Adopt
hako_adopted = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1

source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
generated_artifact_as_native_edit_authority = 0
metadata_refresh_migration = 0
placement_effect_route_collection_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  PlacementEffectTagFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-002
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no route collection migration
no metadata refresh migration
```
