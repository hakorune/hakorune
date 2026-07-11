# 2721 MIRBUILDER-BASIC-BLOCK-TERMINATOR-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for `basic_block_terminator_classifier`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_basic_block_terminator_classifier_parity_gate.sh
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Basic-block terminator classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-BASIC-BLOCK-TERMINATOR-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
