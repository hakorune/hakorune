# 2249 MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-030

Status: Closed
Date: 2026-07-04

## Decision

Select `array_string_len_window_label_formatter` as the thirty-first narrow
Rust-oracle parity pilot owner.

## Reason

`ArrayStringLenWindowMode` and `ArrayStringLenWindowProof` are pure vocabulary
surfaces with stable Rust oracle strings. They do not require migration of
array string len-window matching, string corridor matching, backend lowering,
or MIR mutation.

## Next

`MIRBUILDER-ARRAY-STRING-LEN-WINDOW-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001`
