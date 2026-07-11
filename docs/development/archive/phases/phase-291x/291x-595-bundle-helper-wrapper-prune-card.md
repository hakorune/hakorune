---
Status: Landed
Date: 2026-04-28
Scope: prune the loop_bundle_resolver_v0 helper wrapper shelf after 291x-594 predicate extraction
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/mod.rs
  - src/mir/builder/control_flow/facts/loop_bundle_resolver_v0.rs
  - src/mir/builder/control_flow/facts/loop_bundle_resolver_v0_shape_routes.rs
  - src/mir/builder/control_flow/facts/loop_bundle_resolver_v0_helpers.rs
---

# 291x-595: Bundle Helper Wrapper Prune

## Goal

Remove the now-redundant `loop_bundle_resolver_v0_helpers` shelf and keep the
remaining logic with the owners that actually use it.

This is BoxShape-only cleanup. It does not change accepted loop shapes or
planner behavior.

## Boundaries

- Keep `release_enabled()` with the facts owner that gates extraction.
- Keep shape-local helper logic with `loop_bundle_resolver_v0_shape_routes.rs`.
- Do not change recipe-builder logic or route semantics.

## Result

- Deleted `facts/loop_bundle_resolver_v0_helpers.rs`.
- Moved `release_enabled()` into `loop_bundle_resolver_v0.rs`.
- Moved the remaining shape-only helpers into
  `loop_bundle_resolver_v0_shape_routes.rs`.
- Removed the dead module declaration from `facts/mod.rs`.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
git diff --check
tools/checks/dev_gate.sh quick
```
