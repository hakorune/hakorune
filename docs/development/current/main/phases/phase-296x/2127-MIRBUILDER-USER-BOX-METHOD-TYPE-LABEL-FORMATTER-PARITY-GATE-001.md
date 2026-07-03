# 2127 - MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-PARITY-GATE-001
```

## Purpose

Add the executable parity gate for the sixth narrow Rust-oracle parity pilot:
`user_box_method_type_label_formatter`.

The gate generates a temporary `.hako` EXE app from the Rust-oracle fixture and
diffs normalized type-label output against expected Rust oracle rows.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-user-box-method-type-label-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/user_box_method_type_label_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_user_box_method_type_label_formatter_parity_gate.sh
```

## Acceptance

```text
bash tools/checks/rust_lifecycle_mirbuilder_user_box_method_type_label_formatter_parity_gate.sh
```

Expected output contract:

```text
output_contract=rust-lifecycle-mirbuilder-user-box-method-type-label-formatter-parity-gate-v0
owner=user_box_method_type_label_formatter
parity_rows=10
parity_status=green

source_selfhost_claim = 0
hako_adopted_decision = 0
user_box_method_route_collection_migration = 0
route_convergence_migration = 0
mir_module_mutation_migration = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  UserBoxMethodTypeLabelFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no route collection migration
no route convergence migration
no MirModule mutation migration
no runtime fallback
no new backend route
no new ABI
```
