# 2165 - MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the
`sum_local_aggregate_layout_tag_formatter` narrow parity pilot.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-sum-local-aggregate-layout-tag-formatter-rust-oracle-v0.json
```

## Included Surface

```text
SumLocalAggregateLayout Display text
```

## Excluded Surface

```text
payload-type layout binding
sum placement layout refresh
layout summary formatting
lowering execution
MIR mutation
```

## Acceptance

```text
oracle_row_count = 4
selected_surface_is_pure_enum_tag_formatter = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  SumLocalAggregateLayoutTagFormatterRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no payload-type layout binding migration
no sum placement layout refresh migration
no lowering execution migration
no MIR mutation migration
```
