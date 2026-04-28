---
Status: Landed
Date: 2026-04-28
Scope: prune loop_true_break_continue facts re-export facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/loop_cond/true_break_continue.rs
  - src/mir/builder/control_flow/plan/loop_true_break_continue/mod.rs
---

# 291x-630: Loop True Break Continue Facts Facade Prune

## Goal

Remove the thin `plan::loop_true_break_continue::facts` re-export so callers use
the Facts SSOT directly at `plan::loop_cond::true_break_continue`.

This is BoxShape cleanup only. It does not change accepted loop shapes,
lowering, routing order, or recipe verification behavior.

## Evidence

`loop_true_break_continue::facts` only re-exported:

- `LoopTrueBreakContinueFacts`
- `try_extract_loop_true_break_continue_facts`

The owner implementation already lives in `loop_cond::true_break_continue`, and
the remaining callsites were few enough to point at that owner directly.

## Boundaries

- Remove only the compatibility facts facade.
- Keep `loop_true_break_continue::normalizer` and `loop_true_break_continue::recipe`
  in place.
- Do not move the facts implementation or change extraction behavior.

## Acceptance

- `cargo fmt` completes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Repointed facts callsites to `plan::loop_cond::true_break_continue`.
- Removed the obsolete `loop_true_break_continue::facts` module.

## Verification

```bash
cargo fmt
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
