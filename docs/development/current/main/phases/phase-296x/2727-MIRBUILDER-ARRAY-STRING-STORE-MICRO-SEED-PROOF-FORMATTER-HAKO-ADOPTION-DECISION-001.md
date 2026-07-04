# 2727 MIRBUILDER-ARRAY-STRING-STORE-MICRO-SEED-PROOF-FORMATTER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `array_string_store_micro_seed_proof_formatter` as a narrow HakoAdopted Rust-oracle parity pilot owner after the 1-row `.hako` EXE parity gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-string-store-micro-seed-proof-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_string_store_micro_seed_proof_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_string_store_micro_seed_proof_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-string-store-micro-seed-proof-formatter-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Array/string-store micro seed proof formatting remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-127`
