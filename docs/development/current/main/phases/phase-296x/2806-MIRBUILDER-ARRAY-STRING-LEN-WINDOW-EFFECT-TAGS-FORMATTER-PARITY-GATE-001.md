# 2806 MIRBUILDER-ARRAY-STRING-LEN-WINDOW-EFFECT-TAGS-FORMATTER-PARITY-GATE-001

Status: Completed
Date: 2026-07-05

## Decision

Add a dedicated `.hako` EXE parity gate for `array_string_len_window_effect_tags_formatter`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_string_len_window_effect_tags_formatter_parity_gate.sh
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Array string len-window effect-tags formatting remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-ARRAY-STRING-LEN-WINDOW-EFFECT-TAGS-FORMATTER-HAKO-ADOPTION-DECISION-001`
