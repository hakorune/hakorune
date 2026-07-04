# 2695 MIRBUILDER-SOURCE-TYPE-NAME-TO-MIR-HAKO-NATIVE-IMPLEMENTATION-001

Status: Completed
Date: 2026-07-04

## Decision

Implement `source_type_name_to_mir` in hand-authored `.hako`.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-source-type-name-to-mir-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/source_type_name_to_mir.hako
```

## Non-Claims

- Source Selfhost remains unclaimed.
- MIR type mapping remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-SOURCE-TYPE-NAME-TO-MIR-PARITY-GATE-001`
