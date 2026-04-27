---
Status: Landed
Date: 2026-04-28
Scope: prune cond-profile fact-module forwards from facts/plan_residue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/verify/verifier/cond_profile.rs
  - src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs
  - src/mir/builder/control_flow/plan/facts/bool_predicate_scan_facts.rs
  - src/mir/builder/control_flow/plan/facts/loop_array_join_facts.rs
  - src/mir/builder/control_flow/plan/facts/loop_char_map_facts.rs
---

# 291x-604: Cond-Profile Facts Residue Wrapper Prune

## Goal

Remove the cond-profile fact-module forwards from `facts/plan_residue` after
moving the verifier type imports to the plan facts owner path.

This is BoxShape-only cleanup. It does not change cond-profile acceptance,
freeze behavior, route selection, or lowering.

## Boundaries

- Keep the four fact type owners in `plan/facts/*`.
- Touch only verifier import paths and the residue allowlist.
- Do not move verifier policy or cond-profile logic.

## Result

- Updated `verify/verifier/cond_profile.rs` to import the four fact types from
  `plan::facts`.
- Removed these module forwards from `facts/plan_residue`:
  `accum_const_loop_facts`, `bool_predicate_scan_facts`,
  `loop_array_join_facts`, and `loop_char_map_facts`.

## Verification

```bash
! rg -n "control_flow::facts::\\{[^}]*((accum_const_loop_facts|bool_predicate_scan_facts|loop_array_join_facts|loop_char_map_facts))" src tests -g'*.rs' -U
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
