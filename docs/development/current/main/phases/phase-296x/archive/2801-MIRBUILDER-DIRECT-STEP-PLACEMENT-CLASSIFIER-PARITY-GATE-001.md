# 2801 MIRBUILDER-DIRECT-STEP-PLACEMENT-CLASSIFIER-PARITY-GATE-001

Status: Completed
Date: 2026-07-04

## Decision

Add a dedicated `.hako` EXE parity gate for `direct_step_placement_classifier`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_direct_step_placement_classifier_parity_gate.sh
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Generic-loop direct step placement classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-DIRECT-STEP-PLACEMENT-CLASSIFIER-HAKO-ADOPTION-DECISION-001`
