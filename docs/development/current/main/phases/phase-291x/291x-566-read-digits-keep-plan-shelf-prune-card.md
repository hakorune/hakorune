---
Status: Landed
Date: 2026-04-28
Scope: prune read-digits keep-plan compatibility shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-565-next-lane-selection-card.md
  - src/mir/builder/control_flow/cleanup/policies/loop_true_read_digits_policy.rs
  - src/mir/builder/control_flow/plan/policies/keep_plan/README.md
  - src/mir/builder/control_flow/plan/policies/keep_plan/mod.rs
---

# 291x-566: Read-Digits Keep-Plan Shelf Prune

## Goal

Remove the read-digits policy compatibility file from
`plan/policies/keep_plan/`.

The cleanup-owned policy is already the live owner path, so the keep-plan file
is only a historical re-export shelf.

## Cleaner Boundary

```text
cleanup/policies/loop_true_read_digits_policy.rs
  owns read-digits route classification

plan/loop_break_condition_policy_router.rs
  imports cleanup owner directly

plan/policies/keep_plan/
  does not mirror read-digits policy paths
```

## Boundaries

- BoxShape only.
- Do not change read-digits route classification.
- Do not change `PolicyDecision` semantics.
- Keep P5b and trim shelves untouched in this card.

## Acceptance

- No `keep_plan::loop_true_read_digits` route remains.
- No `plan::policies::*loop_true_read_digits` route remains.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo test --release read_digits_loop_true_policy_returns_break_when_true_and_allowlist --lib` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Deleted the read-digits keep-plan re-export file.
- Removed its module declaration from the keep-plan shelf.
- Updated the keep-plan README to mark read-digits as retired from the compat
  shelf and to point to the cleanup owner path.
- Rechecked the archived read-digits VM smoke against the pre-card baseline:
  it timed out in the baseline too, so it is not used as this prune card's
  acceptance gate.

## Verification

```bash
rg -n "plan::policies::.*loop_true_read_digits|keep_plan::loop_true_read_digits|super::policies::loop_true_read_digits" src/mir/builder/control_flow -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo test --release read_digits_loop_true_policy_returns_break_when_true_and_allowlist --lib
cargo fmt -- --check
git diff --check
```

## Baseline Smoke Note

The archived smoke below currently times out with `RUN_TIMEOUT_SECS=10` both
before and after this prune card:

```bash
bash tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_vm.sh
```

That timeout is a separate strict read-digits VM blocker, not a regression from
removing the unused keep-plan re-export.
