# 2160 - MIRBUILDER-ARRAY-TEXT-OBSERVER-EXECUTOR-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-ARRAY-TEXT-OBSERVER-EXECUTOR-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the
`array_text_observer_executor_tag_formatter` narrow parity pilot.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-array-text-observer-executor-tag-formatter-rust-oracle-v0.json
```

## Included Surface

```text
ArrayTextObserver executor contract enum Display text
```

## Excluded Surface

```text
observer route derivation
region matching
combined region planning
lowering execution
MIR mutation
```

## Acceptance

```text
oracle_row_count = 11
selected_surface_is_pure_enum_tag_formatter = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  ArrayTextObserverExecutorTagFormatterRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-ARRAY-TEXT-OBSERVER-EXECUTOR-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no observer route derivation migration
no region matching migration
no combined region planning migration
no lowering execution migration
no MIR mutation migration
```
