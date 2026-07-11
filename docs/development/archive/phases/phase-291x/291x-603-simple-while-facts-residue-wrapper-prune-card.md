---
Status: Landed
Date: 2026-04-28
Scope: prune the test-only loop_simple_while_facts forwarding module from facts/plan_residue
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/plan_residue.rs
  - src/mir/builder/control_flow/lower/normalize/canonicalize.rs
  - src/mir/builder/control_flow/plan/facts/loop_simple_while_facts.rs
---

# 291x-603: Simple-While Facts Residue Wrapper Prune

## Goal

Remove the `loop_simple_while_facts` forwarding module from
`facts/plan_residue` after moving the lone non-plan caller to the plan facts
owner path.

This is BoxShape-only cleanup. It does not change simple-while extraction,
canonicalization behavior, routing, or lowering.

## Boundaries

- Keep `LoopSimpleWhileFacts` ownership in `plan/facts/loop_simple_while_facts`.
- Touch only the test fixture type path in `lower/normalize/canonicalize.rs`.
- Do not move `LoopFacts` or broader facts modules in this card.

## Result

- Updated the canonicalize test fixture to construct `LoopSimpleWhileFacts`
  through the plan facts owner path.
- Removed `loop_simple_while_facts` from the `facts/plan_residue` allowlist.

## Verification

```bash
! rg -n "control_flow::facts::loop_simple_while_facts" src tests -g'*.rs'
cargo test -q canonical_projects
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
