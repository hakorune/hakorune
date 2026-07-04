# 2622 MIRBUILDER-ARRAY-TEXT-OBSERVER-REGION-CONTRACT-EFFECT-FORMATTER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `array_text_observer_region_contract_effect_formatter` as a narrow
HakoAdopted Rust-oracle parity pilot owner after the 4-row `.hako` EXE parity
gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-observer-region-contract-effect-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_observer_region_contract_effect_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_observer_region_contract_effect_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-observer-region-contract-effect-formatter-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Array-text observer region-contract matching remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-106`
