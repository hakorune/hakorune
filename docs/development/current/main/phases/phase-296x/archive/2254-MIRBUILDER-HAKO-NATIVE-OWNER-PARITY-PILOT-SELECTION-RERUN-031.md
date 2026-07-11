# 2254 MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-031

Status: Closed
Date: 2026-07-04

## Decision

Select `string_direct_set_window_proof_label_formatter` as the thirty-second
narrow Rust-oracle parity pilot owner.

## Reason

`StringDirectSetWindowProof` is a pure one-row vocabulary surface with a stable
Rust oracle string. It does not require migration of string direct-set window
matching, string corridor matching, backend lowering, or MIR mutation.

## Next

`MIRBUILDER-STRING-DIRECT-SET-WINDOW-PROOF-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001`
