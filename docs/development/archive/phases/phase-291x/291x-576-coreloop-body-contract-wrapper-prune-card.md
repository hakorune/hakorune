---
Status: Landed
Date: 2026-04-28
Scope: migrate plan-side core loop body contract callers to verify owner paths and delete the wrapper
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-291x/README.md
  - src/mir/builder/control_flow/verify/coreloop_body_contract.rs
  - src/mir/builder/control_flow/plan/mod.rs
---

# 291x-576: Coreloop Body Contract Wrapper Prune

## Goal

Remove the `plan::coreloop_body_contract` compatibility wrapper now that its
remaining callers can import the verify-owned contract directly.

This cleanup keeps the body-effect contract owned by `verify/` and prevents the
plan layer from re-exporting another verification seam.

## Evidence

Before the prune, only six live references remained:

- five `use ...::is_effect_only_stmt` imports inside generic-loop analysis and
  lowering helpers
- one `has_control_flow_effect` call in `plan/lowerer/body_processing.rs`

After the migration, those callers import the verify owner directly:

```text
crate::mir::builder::control_flow::verify::coreloop_body_contract
```

## Cleaner Boundary

```text
verify/coreloop_body_contract.rs
  owns the core loop body effect contract

plan/*
  may consume the contract, but no longer republishes it
```

## Boundaries

- Migrate only the remaining wrapper callers to the verify owner path.
- Delete the unused plan-side wrapper module declaration and file.
- Do not change loop acceptance, lowering behavior, or the contract helpers
  themselves.
- Keep follow-up queue work (`plan/facts` wrappers and owner-path migrations) as
  separate cards.

## Acceptance

- No `plan::coreloop_body_contract` users remain in `src/` or `tests/`.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Migrated the remaining generic-loop and lowerer callers to the verify owner
  path.
- Removed `src/mir/builder/control_flow/plan/coreloop_body_contract.rs`.
- Removed the `plan/mod.rs` module declaration for the deleted wrapper.
- Advanced the thin restart mirrors to the next queue item: unused `plan/facts`
  wrappers.

## Verification

```bash
rg -n "plan::coreloop_body_contract|control_flow::plan::coreloop_body_contract" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
