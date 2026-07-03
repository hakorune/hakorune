# 2178 - MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `array_text_state_residence_label_formatter` as the sixteenth narrow
Rust-oracle parity pilot owner after a green 5-row `.hako` EXE parity gate.

This decision adopts only pure array/text state-residence contract label
formatting. It does not adopt route matching, exact-shape payload construction,
executor/session planning, backend lowering, MIR mutation, Source Selfhost, or
full MirBuilder conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-state-residence-label-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_state_residence_label_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_state_residence_label_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-state-residence-label-formatter-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 5
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
array_text_route_matching_migration = 0
exact_shape_payload_migration = 0
executor_planning_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  ArrayTextStateResidenceLabelFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-016
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no array/text route matching migration
no exact-shape payload migration
no executor/session planning migration
no backend lowering migration
no MIR mutation migration
```
