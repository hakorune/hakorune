---
Status: Landed
Date: 2026-04-28
Scope: prune cleanup top-level wildcard re-exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/cleanup/mod.rs
---

# 291x-618: Cleanup Top-Level Wildcard Prune

## Goal

Remove unused wildcard re-exports from `control_flow::cleanup`. Live callers
already import the concrete `cleanup::common` or `cleanup::policies` owner
modules directly.

This is BoxShape-only cleanup. It does not change cleanup helper behavior,
policy decisions, accepted shapes, or lowering behavior.

## Boundaries

- Keep `cleanup::common` and `cleanup::policies` modules intact.
- Do not move policy implementations in this card.
- Do not change any call sites because they already use concrete owner paths.

## Result

- Removed `pub use common::*` and `pub use policies::*` from `cleanup/mod.rs`.
- Updated the module comment to point callers at concrete owner modules.

## Verification

```bash
! rg -n "control_flow::cleanup::(decide_carrier_binding_policy|CarrierBindingPolicy|get_entry_function|PolicyDecision|BalancedDepthScanPolicyBox)|cleanup::\\*" src/mir/builder/control_flow src/mir/builder -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
