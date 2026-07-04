# 2587 MIRBUILDER-ARRAY-TEXT-COMBINED-REGION-EXECUTION-MODE-FORMATTER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-04

## Decision

Adopt `array_text_combined_region_execution_mode_formatter` as a narrow
HakoAdopted Rust-oracle parity pilot owner after the 1-row `.hako` EXE parity
gate is green.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-combined-region-execution-mode-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_text_combined_region_execution_mode_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_combined_region_execution_mode_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-text-combined-region-execution-mode-formatter-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Array-text combined region planning remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-100`
