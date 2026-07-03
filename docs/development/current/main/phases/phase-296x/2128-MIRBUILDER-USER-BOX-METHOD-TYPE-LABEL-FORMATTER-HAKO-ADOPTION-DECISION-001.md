# 2128 - MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `user_box_method_type_label_formatter` as the sixth narrow Rust-oracle
parity pilot owner after a green 10-row `.hako` EXE parity gate.

This decision adopts only the pure formatter:

```text
MirType -> user-box method route type label
```

It does not adopt user-box method route collection, route convergence,
materialization fixpoint, MirModule mutation, backend emission, Source
Selfhost, or full MirBuilder conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-user-box-method-type-label-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/user_box_method_type_label_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_user_box_method_type_label_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-user-box-method-type-label-formatter-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 10
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
user_box_method_route_collection_migration = 0
route_convergence_migration = 0
materialization_fixpoint_migration = 0
mir_module_mutation_migration = 0
backend_emission_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  UserBoxMethodTypeLabelFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-006
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
no route convergence migration
no MirModule mutation migration
no backend emission migration
```
