---
Status: Landed
Date: 2026-04-28
Scope: prune trim keep-plan compatibility shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/cleanup/policies/trim_policy.rs
  - src/mir/builder/control_flow/plan/trim_loop_lowering.rs
  - src/mir/builder/control_flow/plan/policies/keep_plan/README.md
  - src/mir/builder/control_flow/plan/policies/keep_plan/mod.rs
---

# 291x-568: Trim Keep-Plan Shelf Prune

## Goal

Remove the trim policy compatibility file from `plan/policies/keep_plan/`.

The trim classifier already lives under `cleanup/policies/trim_policy.rs`, and
the only remaining plan-policy dependency is the `PolicyDecision` import in
`trim_loop_lowering.rs`.

## Cleaner Boundary

```text
cleanup/policies/trim_policy.rs
  owns trim route classification and PolicyDecision use

plan/trim_loop_lowering.rs
  imports cleanup policy owner directly

plan/policies/keep_plan/
  does not mirror trim policy paths
```

## Boundaries

- BoxShape only.
- Do not change trim route classification.
- Do not change `PolicyDecision` semantics.
- Keep P5b shelf untouched in this card.

## Acceptance

- No `keep_plan::trim_policy` route remains.
- No `plan::policies::*trim_policy` route remains.
- No `super::policies::PolicyDecision` import remains in `trim_loop_lowering.rs`.
- No unused `PolicyDecision` re-export remains in the keep-plan shelf.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes without targeted warning.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Moved `trim_loop_lowering.rs`'s `PolicyDecision` import to the cleanup policy
  owner path.
- Deleted the trim keep-plan re-export file.
- Removed the now-unused keep-plan `PolicyDecision` re-export.
- Updated the keep-plan README to mark the trim shelf retired.

## Verification

```bash
rg -n "plan::policies::.*trim_policy|keep_plan::trim_policy|super::policies::trim_policy|super::policies::PolicyDecision" src/mir/builder/control_flow -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
