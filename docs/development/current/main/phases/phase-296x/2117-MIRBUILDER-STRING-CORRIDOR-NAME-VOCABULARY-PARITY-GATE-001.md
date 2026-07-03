# 2117 - MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-PARITY-GATE-001

## Token

```text
MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-PARITY-GATE-001
```

## Purpose

Add the executable parity gate for the fourth narrow Rust-oracle parity pilot:
`string_corridor_name_vocabulary_classifier`.

The gate generates a temporary `.hako` EXE app from the Rust-oracle fixture and
diffs normalized `1` / `0` category output against expected vocabulary tags.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-string-corridor-name-vocabulary-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/string_corridor_name_vocabulary.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_string_corridor_name_vocabulary_parity_gate.sh
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_string_corridor_name_vocabulary_parity_gate.sh
```

Expected output contract:

```text
output_contract=rust-lifecycle-mirbuilder-string-corridor-name-vocabulary-parity-gate-v0
owner=string_corridor_name_vocabulary_classifier
parity_rows=18
parity_status=green

source_selfhost_claim = 0
hako_adopted_decision = 0
string_corridor_fact_inference_migration = 0
mir_instruction_traversal_migration = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  StringCorridorNameVocabularyRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-STRING-CORRIDOR-NAME-VOCABULARY-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no string corridor fact inference migration
no MIR instruction traversal migration
no runtime fallback
no new backend route
no new ABI
```
