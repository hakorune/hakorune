---
Status: Landed implementation card
Date: 2026-04-24
Scope: Quiet intentional Rust dead-code warnings without deleting staged compiler seams.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/README.md
---

# 291x-126 Rust Warning Quieting Card

## Goal

Make the normal Rust library check quiet for warnings that are already known to
be intentional staged seams.

This is a BoxShape / hygiene card. It does not add a new CoreBox row, route
shape, parser rule, runtime behavior, environment variable, or fallback.

## Contract

- `cargo check --lib` should emit no warnings from the current crate.
- Suppression must stay local to the module or item that owns the staged seam.
- Do not add crate-wide `#![allow(dead_code)]`.
- Do not delete route vocabulary, recipe vocabulary, verifier hooks, or bridge
  code just to silence warnings.
- If a warning is a true duplicate / unused artifact, remove the duplicate
  instead of suppressing it.

## Quieted Buckets

- control-flow recognizer / recipe vocabulary that is intentionally ahead of
  active routes
- debug-only verifier probes exported for targeted tests and future hooks
- exit-binding modularization kept as an owner-local seam
- Stage1 Program(JSON v0) test helper wrapper
- hako-ll compare bridge modules kept behind explicit recipe routing
- K2 kernel no-perf observe shims and value-codec compatibility helpers
- duplicate string-corridor publish extern constants in the concat corridor
  submodule

## Non-Goals

- enabling new route shapes
- deleting staged compiler boxes
- changing MIR / VM / LLVM behavior
- hiding all future warnings globally
- making `cargo test --lib` pass globally; existing test failures are outside
  this card

## Acceptance

```bash
cargo fmt -- --check
RUSTFLAGS="-D warnings" cargo check --lib
RUSTFLAGS="-D warnings" cargo test -p nyash_kernel runtime_data_invalid_handle_returns_zero
cargo test string_surface_catalog --lib
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
