# 2807 MIRBUILDER-ARRAY-STRING-LEN-WINDOW-EFFECT-TAGS-FORMATTER-HAKO-ADOPTION-DECISION-001

Status: Completed
Date: 2026-07-05

## Decision

Adopt `array_string_len_window_effect_tags_formatter` as a narrow HakoAdopted Rust-oracle parity pilot owner after the green 3-row `.hako` EXE parity gate.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-string-len-window-effect-tags-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/array_string_len_window_effect_tags_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_string_len_window_effect_tags_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-array-string-len-window-effect-tags-formatter-hako-adoption-decision-v0.json
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Array string len-window effect-tags formatting remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-143`
