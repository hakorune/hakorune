# 291x-739 Inline Boundary Builder Test Surface Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/inline_boundary_builder.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`JoinInlineBoundaryBuilder` no longer has production callers. The remaining
callers are contract-check tests that use the builder as a concise fixture
constructor. Keeping the module and public re-export in normal builds made a
test helper look like a supported lowering construction API.

## Decision

Keep the builder as test-only surface. Do not delete it yet, because it still
keeps boundary contract tests readable. Hide both the module and re-export from
non-test builds so `JoinInlineBoundary` construction in production remains
explicit and owned by the actual lowering/merge paths.

## Changes

- Gated `inline_boundary_builder` behind `#[cfg(test)]`.
- Gated the `JoinInlineBoundaryBuilder` re-export behind `#[cfg(test)]`.
- Advanced `CURRENT_STATE.toml` to 291x-739.

## Proof

- `rg -n "JoinInlineBoundaryBuilder" src/mir/builder src/mir/join_ir tests -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
