---
Status: Landed
Date: 2026-04-28
Scope: prune the loop_scan_phi_vars_v0 helper wrapper shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/mod.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0_shape_routes.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0_helpers.rs
---

# 291x-599: Phi Helper Wrapper Prune

## Goal

Remove the `loop_scan_phi_vars_v0_helpers` shelf and return its helpers to the
facts owner and the shape-route owner that actually consume them.

This is BoxShape-only cleanup. It does not change route acceptance or recipe
construction.

## Boundaries

- Keep release gating and nested-loop recipe construction in
  `loop_scan_phi_vars_v0.rs`.
- Keep local shape predicates in `loop_scan_phi_vars_v0_shape_routes.rs`.
- Reuse existing shared predicates instead of reintroducing wrappers.

## Result

- Deleted `facts/loop_scan_phi_vars_v0_helpers.rs`.
- Moved fact-owned helpers into `loop_scan_phi_vars_v0.rs`.
- Moved shape-only predicates into `loop_scan_phi_vars_v0_shape_routes.rs`.
- Removed the dead module declaration from `facts/mod.rs`.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
git diff --check
tools/checks/dev_gate.sh quick
```
