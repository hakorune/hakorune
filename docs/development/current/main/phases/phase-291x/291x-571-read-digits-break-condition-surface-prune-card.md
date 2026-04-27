---
Status: Landed
Date: 2026-04-28
Scope: prune unused read-digits break-condition plan-side compatibility surface
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/cleanup/policies/mod.rs
  - src/mir/builder/control_flow/cleanup/policies/read_digits_break_condition_box.rs
  - src/mir/builder/control_flow/plan/mod.rs
---

# 291x-571: Read-Digits Break-Condition Surface Prune

## Goal

Remove the unused `plan/read_digits_break_condition_box.rs` compatibility
surface now that the read-digits policy imports the cleanup owner directly.

This is a BoxShape cleanup: it does not add a new accepted form or change
lowering behavior.

## Evidence

Before the prune, the only non-owner references to the compatibility surface
were its module declaration and the re-export itself:

```bash
rg -n "plan::read_digits_break_condition_box|control_flow::plan::read_digits_break_condition_box|super::read_digits_break_condition_box|read_digits_break_condition_box::ReadDigitsBreakConditionBox" src tests -g'*.rs'
```

The live route already imports the cleanup owner:

```text
src/mir/builder/control_flow/cleanup/policies/loop_true_read_digits_policy.rs
```

## Cleaner Boundary

```text
cleanup/policies/read_digits_break_condition_box.rs
  owns the read-digits break-condition extractor

cleanup/policies/loop_true_read_digits_policy.rs
  imports the owner directly

plan/
  no longer publishes a compatibility shelf for this extractor
```

## Boundaries

- Delete the unused plan-side re-export and module declaration only.
- Do not change the read-digits extractor implementation.
- Do not change read-digits route acceptance or lowering behavior.
- Refresh cleanup policy owner wording so new callers do not reintroduce a
  plan-side shelf.

## Acceptance

- No `plan::read_digits_break_condition_box` users remain.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.
- `bash tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_vm.sh` passes.

## Result

- Removed `src/mir/builder/control_flow/plan/read_digits_break_condition_box.rs`.
- Removed the stale module declaration from `plan/mod.rs`.
- Updated cleanup policy owner comments to direct callers to owner modules
  rather than plan-side compatibility shelves.

## Verification

```bash
rg -n "plan::read_digits_break_condition_box|control_flow::plan::read_digits_break_condition_box|super::read_digits_break_condition_box" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
bash tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_vm.sh
```
