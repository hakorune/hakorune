# 2175 - MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Record the 5-row Rust-oracle fixture for
`array_text_state_residence_label_formatter`.

The fixture covers only the narrow array/text state-residence contract labels.
It does not adopt route matching, exact-shape payload construction, executor
planning, backend lowering, or MIR mutation.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-state-residence-label-formatter-rust-oracle-v0.json
```

## Acceptance

```text
fixture.kind = MirBuilderArrayTextStateResidenceLabelFormatterRustOracleV1
row_count = 5

source_selfhost_claim = 0
hako_adopted_decision = 0
array_text_route_matching_migration = 0
exact_shape_payload_migration = 0
executor_planning_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

selected_next_card:
  MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```
