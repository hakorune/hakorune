# 2122 - MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-PARITY-GATE-001
```

## Purpose

Add the executable parity gate for the fifth narrow Rust-oracle parity pilot:
`same_module_definition_kind_formatter`.

The gate generates a temporary `.hako` EXE app from the Rust-oracle fixture and
diffs normalized JSON-name output against expected Rust oracle rows.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-same-module-definition-kind-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/same_module_definition_kind_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_same_module_definition_kind_formatter_parity_gate.sh
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_same_module_definition_kind_formatter_parity_gate.sh
```

Expected output contract:

```text
output_contract=rust-lifecycle-mirbuilder-same-module-definition-kind-formatter-parity-gate-v0
owner=same_module_definition_kind_formatter
parity_rows=2
parity_status=green

source_selfhost_claim = 0
hako_adopted_decision = 0
same_module_definition_closure_migration = 0
mir_module_mutation_migration = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  SameModuleDefinitionKindFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no same-module definition closure migration
no route traversal migration
no MirModule mutation migration
no runtime fallback
no new backend route
no new ABI
```
