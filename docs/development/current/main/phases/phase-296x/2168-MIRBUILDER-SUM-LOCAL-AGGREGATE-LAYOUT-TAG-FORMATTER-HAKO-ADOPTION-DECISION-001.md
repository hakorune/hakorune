# 2168 - MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `sum_local_aggregate_layout_tag_formatter` as the fourteenth narrow
Rust-oracle parity pilot owner after a green 4-row `.hako` EXE parity gate.

This decision adopts only the pure SumLocalAggregateLayout Display surface. It
does not adopt payload-type layout binding, sum placement layout refresh,
layout summary formatting, lowering execution, MIR mutation, Source Selfhost,
or full MirBuilder conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-sum-local-aggregate-layout-tag-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/sum_local_aggregate_layout_tag_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_sum_local_aggregate_layout_tag_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-sum-local-aggregate-layout-tag-formatter-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 4
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
payload_type_layout_binding_migration = 0
sum_placement_layout_refresh_migration = 0
lowering_execution_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  SumLocalAggregateLayoutTagFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-014
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no payload-type layout binding migration
no sum placement layout refresh migration
no lowering execution migration
no MIR mutation migration
```
