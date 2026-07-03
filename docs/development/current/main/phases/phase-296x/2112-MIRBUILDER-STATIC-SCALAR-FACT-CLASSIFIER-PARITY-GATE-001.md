# 2112 - MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-PARITY-GATE-001

## Token

```text
MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-PARITY-GATE-001
```

## Purpose

Add the executable parity gate for the third narrow Rust-oracle parity pilot:
`static_scalar_fact_classifier`.

The gate generates a temporary `.hako` EXE app from the Rust-oracle fixture and
diffs normalized output against expected static-scalar fact text.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-static-scalar-fact-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/static_scalar_fact_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_static_scalar_fact_classifier_parity_gate.sh
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_static_scalar_fact_classifier_parity_gate.sh
```

Expected output contract:

```text
output_contract=rust-lifecycle-mirbuilder-static-scalar-fact-classifier-parity-gate-v0
owner=static_scalar_fact_classifier
parity_rows=8
parity_status=green

source_selfhost_claim = 0
hako_adopted_decision = 0
mirbuilder_state_mutation_migration = 0
emit_static_scalar_fact_const_migration = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  StaticScalarFactClassifierRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no MirBuilder state mutation migration
no runtime fallback
no new backend route
no new ABI
```
