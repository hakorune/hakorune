---
Status: Landed
Date: 2026-04-28
Scope: prune plan facts if_phi_join submodule shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/facts/mod.rs
  - src/mir/builder/control_flow/plan/facts/if_phi_join_facts.rs
---

# 291x-611: If Phi Join Facts Submodule Shelf Prune

## Goal

Remove the pure compatibility submodule
`plan::facts::if_phi_join_facts` while keeping the live top-level
`plan::facts::{IfPhiJoinFacts, try_extract_if_phi_join_facts}` surface intact.

This is BoxShape-only cleanup. It does not change if-phi-join extraction,
recipe construction, accepted loop shapes, or lowering behavior.

## Boundaries

- Keep the implementation owner at `control_flow::facts::if_phi_join_facts`.
- Preserve the existing `plan::facts` top-level names used by plan-side callers.
- Do not change `LoopFacts` fields or recipe builder inputs in this card.

## Result

- Removed `mod if_phi_join_facts` from `plan::facts`.
- Re-exported `try_extract_if_phi_join_facts` directly from the facts owner.
- Replaced the `IfPhiJoinFacts` alias to point directly at the facts owner.
- Deleted the now-empty compatibility submodule file.

## Verification

```bash
! rg -n "plan::facts::if_phi_join_facts|mod if_phi_join_facts" src/mir/builder/control_flow/plan -g'*.rs'
cargo test -q if_phi_join
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
