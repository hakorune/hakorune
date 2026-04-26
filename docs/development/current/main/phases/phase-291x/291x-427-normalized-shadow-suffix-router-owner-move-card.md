---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow suffix router owner move
Related:
  - src/mir/builder/control_flow/normalization/suffix_router_box.rs
  - src/mir/builder/control_flow/normalization/mod.rs
  - src/mir/builder/control_flow/cleanup/policies/mod.rs
  - src/mir/builder/stmts/block_stmt.rs
  - docs/development/current/main/phases/phase-291x/291x-426-normalized-shadow-suffix-router-owner-inventory-card.md
---

# 291x-427: Normalized-Shadow Suffix Router Owner Move

## Goal

Move the normalized-shadow suffix router under the normalization owner.

This is a BoxShape cleanup. No route behavior changed.

## Change

Moved:

```text
src/mir/builder/control_flow/cleanup/policies/normalized_shadow_suffix_router_box.rs
  -> src/mir/builder/control_flow/normalization/suffix_router_box.rs
```

Updated the caller to import the router from the normalization facade:

```rust
crate::mir::builder::control_flow::normalization::NormalizedShadowSuffixRouterBox
```

Closed the now-unused `route_entry::policies` compatibility re-export because
the suffix router was its last caller and keeping it produced an unused-import
warning.

The router still delegates to:

```text
NormalizationPlanBox
NormalizationExecuteBox
PlanKind::LoopOnly
```

## Preserved Behavior

- `PlanKind::LoopOnly` is unchanged.
- `consumed = 1` is unchanged.
- strict/non-strict decline behavior is unchanged.
- `[normalization/fallback]` debug tag is unchanged.
- route order in `build_block()` is unchanged.
- `NormalizationPlanBox` detection logic is unchanged.
- `NormalizationExecuteBox` lowering/merge logic is unchanged.
- route-entry compatibility policy re-export removal is behavior-neutral; no
  caller path remained.

## Verification

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "route_entry::policies::normalized_shadow_suffix_router_box|cleanup::policies::normalized_shadow_suffix_router_box" \
  src/mir/builder -g '*.rs'
rg -n "NormalizedShadowSuffixRouterBox" \
  src/mir/builder/control_flow/normalization src/mir/builder/stmts/block_stmt.rs -g '*.rs'
```

## Next Cleanup

Inventory the next compiler-cleanliness seam after normalized-shadow suffix
router owner move.
