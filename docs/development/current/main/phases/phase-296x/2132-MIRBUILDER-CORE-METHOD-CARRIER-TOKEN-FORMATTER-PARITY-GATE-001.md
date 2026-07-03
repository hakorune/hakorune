# 2132 - MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-PARITY-GATE-001
```

## Purpose

Add the Rust-oracle parity gate for the hand-authored
`core_method_carrier_token_formatter` `.hako` implementation.

## Gate

```text
tools/checks/rust_lifecycle_mirbuilder_core_method_carrier_token_formatter_parity_gate.sh
```

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-core-method-carrier-token-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/core_method_carrier_token_formatter.hako
```

## Acceptance

```text
output_contract =
  rust-lifecycle-mirbuilder-core-method-carrier-token-formatter-parity-gate-v0

parity_rows = 32
parity_status = green
source_selfhost_claim = 0
hako_adopted_decision = 0

core_method_contract_manifest_migration = 0
method_resolution_migration = 0
route_collection_migration = 0
lowering_execution_migration = 0
backend_emission_migration = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  CoreMethodCarrierTokenFormatterParityGateGreen

selected_next_card:
  MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no manifest migration
no method resolution migration
no carrier route collection migration
no lowering execution migration
no backend emission migration
```
