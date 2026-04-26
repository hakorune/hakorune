---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow suffix router owner inventory
Related:
  - src/mir/builder/control_flow/cleanup/policies/normalized_shadow_suffix_router_box.rs
  - src/mir/builder/control_flow/cleanup/policies/mod.rs
  - src/mir/builder/control_flow/normalization/mod.rs
  - src/mir/builder/stmts/block_stmt.rs
  - docs/development/current/main/phases/phase-291x/291x-425-normalized-shadow-public-surface-prune-card.md
---

# 291x-426: Normalized-Shadow Suffix Router Owner Inventory

## Goal

Pick the next small compiler-cleanliness seam after normalized-shadow public
surface prune.

This is a BoxShape inventory. No behavior changed.

## Findings

`NormalizedShadowSuffixRouterBox` is physically located under
`control_flow/cleanup/policies`, and `block_stmt.rs` imports it through the
route-entry compatibility policy path:

```rust
control_flow::joinir::route_entry::policies::normalized_shadow_suffix_router_box
```

The router implementation no longer owns cleanup policy decisions. It delegates
to the normalization owner:

```text
NormalizationPlanBox    = shape/consumed decision
NormalizationExecuteBox = lowering/merge execution
PlanKind::LoopOnly      = current accepted normalization unit
```

This makes the current physical path a responsibility mismatch:

```text
cleanup/policies/normalized_shadow_suffix_router_box.rs
  -> normalization planner/executor owner
```

## Decision

Move the suffix router module under `control_flow/normalization` and import it
from the normalization facade.

Do not change:

- `PlanKind::LoopOnly`
- `consumed = 1`
- strict/non-strict decline behavior
- `[normalization/fallback]` debug tag
- route order in `build_block()`
- `NormalizationPlanBox` detection logic
- `NormalizationExecuteBox` lowering/merge logic

## Next Cleanup

`291x-427`: normalized-shadow suffix router owner move.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "route_entry::policies::normalized_shadow_suffix_router_box|cleanup::policies::normalized_shadow_suffix_router_box" \
  src/mir/builder -g '*.rs'
rg -n "NormalizedShadowSuffixRouterBox" \
  src/mir/builder/control_flow/normalization src/mir/builder/stmts/block_stmt.rs -g '*.rs'
```

The first `rg` should produce no output. The second should show only the new
normalization owner path and the caller.
