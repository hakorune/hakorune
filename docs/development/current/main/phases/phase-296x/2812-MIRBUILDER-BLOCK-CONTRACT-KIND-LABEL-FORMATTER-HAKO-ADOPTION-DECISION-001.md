# 2812 - MIRBUILDER-BLOCK-CONTRACT-KIND-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-05

## Decision

Adopt `block_contract_kind_label_formatter` as a narrow HakoAdopted Rust-oracle parity pilot owner after the green 4-row `.hako` EXE parity gate.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-block-contract-kind-label-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/block_contract_kind_label_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_block_contract_kind_label_formatter_parity_gate.sh
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- VerifiedRecipeBlock kind vocabulary remains Rust.
- Backend lowering and MIR mutation remain Rust.
