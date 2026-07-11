# 291x-790 Cond Profile Allow Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/policies/cond_profile.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

After the dead shelf sweep and wrapper cleanup, the remaining active-hold audit
showed that `cond_profile.rs` no longer needed its file-level dead-code allow.
Repository evidence found live runtime usage for the full active surface:

- `CondSkeleton`
- `CondParam`
- `CmpOp`
- `BoundExpr`
- `StepExpr`
- `CondProfile`
- `CondProfile::new`
- `CondProfile::loop_var_name`

## Decision

Delete the file-level `#![allow(dead_code)]` from `cond_profile.rs` and keep
`hints.rs` as the only remaining broad scaffold hold in this audited set.

## Landed

- Removed the file-level dead-code allow from `src/mir/policies/cond_profile.rs`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

Within the originally audited MIR structural vocabulary set:

- `cond_profile` is now clean without a broad hold
- `hints` remains an intentional scaffold hold

This leaves `hints.rs` as the only broad hold still justified in the narrowed
MIR vocabulary audit.

## Proof

- `rg -n "allow\\(dead_code\\)" src/mir/policies/cond_profile.rs src/mir/hints.rs`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
