---
Status: Landed
Date: 2026-04-28
Scope: prune the loop_scan_methods_block_v0 helper wrapper shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/mod.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0_shape_routes.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0_recipe_builder.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0_helpers.rs
---

# 291x-600: Methods-Block Helper Wrapper Prune

## Goal

Remove the `loop_scan_methods_block_v0_helpers` shelf and return its helpers to
the facts owner, shape matcher, and recipe-builder owners that already own the
behavior.

This is BoxShape-only cleanup. It does not change accepted shapes or segment
construction policy.

## Boundaries

- Keep release gating and condition extraction in
  `loop_scan_methods_block_v0.rs`.
- Keep shape-only scan-window recognizers in
  `loop_scan_methods_block_v0_shape_routes.rs`.
- Keep statement flattening and segment building in
  `loop_scan_methods_block_v0_recipe_builder.rs`.

## Result

- Deleted `facts/loop_scan_methods_block_v0_helpers.rs`.
- Moved fact-owned helpers into `loop_scan_methods_block_v0.rs`.
- Moved shape-only recognizers into `loop_scan_methods_block_v0_shape_routes.rs`.
- Moved segment-builder helpers into
  `loop_scan_methods_block_v0_recipe_builder.rs`.
- Removed the dead module declaration from `facts/mod.rs`.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
git diff --check
tools/checks/dev_gate.sh quick
```
