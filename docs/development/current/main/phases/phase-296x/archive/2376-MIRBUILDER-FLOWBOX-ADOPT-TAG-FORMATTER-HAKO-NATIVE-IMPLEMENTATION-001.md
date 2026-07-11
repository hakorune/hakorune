# 2376 MIRBUILDER-FLOWBOX-ADOPT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Add the hand-authored `.hako` implementation for `flowbox_adopt_tag_formatter`.

## Boundary

The implementation accepts scalar `box_kind`, `features_csv`, and `via_label`
values and returns a single Rust-oracle tag string.

## Non-Claims

- No generated artifact as native edit authority.
- No stderr emission migration.

## Next

`MIRBUILDER-FLOWBOX-ADOPT-TAG-FORMATTER-PARITY-GATE-001`
