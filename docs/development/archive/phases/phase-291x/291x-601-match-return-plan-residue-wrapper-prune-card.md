---
Status: Landed
Date: 2026-04-28
Scope: prune the match-return forwarding pair from facts/plan_residue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/stmts/return_stmt.rs
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/plan/facts/match_return_facts.rs
---

# 291x-601: Match-Return Plan Residue Wrapper Prune

## Goal

Remove the match-return forwarding pair from `facts/plan_residue` after moving
the lone non-plan caller to the plan facts owner path.

This is BoxShape-only cleanup. It does not change match-return acceptance,
strict/release adoption, CorePlan composition, or lowering.

## Boundaries

- Keep match-return facts ownership in `plan/facts/match_return_facts.rs`.
- Do not add new `plan::facts` top-level exports.
- Keep `facts/plan_residue.rs` as a narrow allowlist for remaining live
  non-plan callers only.

## Result

- Updated `stmts/return_stmt.rs` to import
  `try_extract_match_return_facts` and `MatchReturnFacts` from the plan facts
  owner path.
- Removed the now-unused match-return forwarding pair from
  `facts/plan_residue.rs`.

## Verification

```bash
! rg -n "control_flow::facts::.*(try_extract_match_return_facts|MatchReturnFacts)" src tests -g'*.rs'
cargo test -q match_return_facts
cargo check --release --bin hakorune -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
