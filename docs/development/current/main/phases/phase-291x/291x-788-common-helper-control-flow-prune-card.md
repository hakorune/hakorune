# 291x-788 Common Helper Control-Flow Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/control_flow/facts/extractors/common_helpers/control_flow.rs`
- `src/mir/builder/control_flow/facts/extractors/common_helpers/mod.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

After `291x-787` retired the dead `loop_simple_while` extractor shelf, the
shared helper layer still exported one now-unused residue:

- `has_control_flow_statement`

Repository search found no remaining `src/` or `tests/` callers, and the proof
bundle surfaced it as the only new dead-code warning.

## Decision

Delete `has_control_flow_statement` and remove its re-export from the common
helper owner surface.

## Landed

- Removed the unused `has_control_flow_statement` helper.
- Removed its re-export from `common_helpers/mod.rs`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The `loop_simple_while` cleanup no longer leaves a helper-layer residue. The
remaining MIR structural vocabulary queue is now limited to active holds or new
owner-backed seams only.

## Proof

- `rg -n "has_control_flow_statement" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
