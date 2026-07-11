# 2290 MIRBUILDER-ARRAY-TEXT-EDIT-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001

Status: Completed
Date: 2026-07-04

## Decision

Create the 4-row Rust-oracle fixture for
`array_text_edit_label_formatter`.

## Fixture

`docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-array-text-edit-label-formatter-rust-oracle-v0.json`

## Scope

- Rows cover edit kind, split policy, and proof label vocabulary only.
- Rust source remains the oracle.
- Matching, payload migration, backend action execution, and MIR mutation stay
  outside this owner.

## Next

`MIRBUILDER-ARRAY-TEXT-EDIT-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001`
