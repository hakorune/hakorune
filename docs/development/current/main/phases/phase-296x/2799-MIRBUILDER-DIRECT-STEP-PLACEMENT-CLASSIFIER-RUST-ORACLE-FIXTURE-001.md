# 2799 MIRBUILDER-DIRECT-STEP-PLACEMENT-CLASSIFIER-RUST-ORACLE-FIXTURE-001

Status: Completed
Date: 2026-07-04

## Decision

Create a 4-row Rust oracle fixture for `direct_step_placement_classifier`.

## Evidence

```text
owner:
  direct_step_placement_classifier
source_surface:
  src/mir/builder/control_flow/generic_loop_canon/step_placement/plan.rs:51
  classify_direct_indices
```

## Non-Claims

- Source Selfhost remains unclaimed.
- Rust bootstrap/oracle remains retained.
- Generic-loop direct step placement classification remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-DIRECT-STEP-PLACEMENT-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001`
