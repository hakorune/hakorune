# 2155 - MIRBUILDER-THIN-ENTRY-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-THIN-ENTRY-TAG-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the `thin_entry_tag_formatter` narrow parity
pilot.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-thin-entry-tag-formatter-rust-oracle-v0.json
```

## Included Surface

```text
ThinEntrySurface Display text
ThinEntryPreferredEntry Display text
ThinEntryCurrentCarrier Display text
ThinEntryValueClass Display text
ThinEntryDemand Display text
```

## Excluded Surface

```text
thin-entry candidate collection
thin-entry selection
manifest generation
lowering execution
MIR mutation
```

## Acceptance

```text
oracle_row_count = 23
selected_surface_is_pure_enum_tag_formatter = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  ThinEntryTagFormatterRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-THIN-ENTRY-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no thin-entry candidate collection migration
no thin-entry selection migration
no manifest generation migration
no lowering execution migration
no MIR mutation migration
```
