# 2295 MIRBUILDER-STRING-CORRIDOR-RELATION-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001

Status: Completed
Date: 2026-07-04

## Decision

Create the 4-row Rust-oracle fixture for
`string_corridor_relation_label_formatter`.

## Fixture

`docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-string-corridor-relation-label-formatter-rust-oracle-v0.json`

## Scope

- Rows cover relation kind and window contract label vocabulary only.
- Rust source remains the oracle.
- PHI relation collection, relation detection, window policy, and MIR mutation
  stay outside this owner.

## Next

`MIRBUILDER-STRING-CORRIDOR-RELATION-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001`
