# 2697 MIRBUILDER-SOURCE-TYPE-NAME-TO-MIR-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `source_type_name_to_mir` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 9-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-source-type-name-to-mir-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/source_type_name_to_mir.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_source_type_name_to_mir_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-source-type-name-to-mir-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- MIR type mapping remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-122`
