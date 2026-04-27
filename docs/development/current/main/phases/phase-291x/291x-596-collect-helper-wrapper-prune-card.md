---
Status: Landed
Date: 2026-04-28
Scope: prune the loop_collect_using_entries_v0 helper wrapper shelf after 291x-594 predicate extraction
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/mod.rs
  - src/mir/builder/control_flow/facts/loop_collect_using_entries_v0.rs
  - src/mir/builder/control_flow/facts/loop_collect_using_entries_v0_shape_routes.rs
  - src/mir/builder/control_flow/facts/loop_collect_using_entries_v0_helpers.rs
---

# 291x-596: Collect Helper Wrapper Prune

## Goal

Remove the now-redundant `loop_collect_using_entries_v0_helpers` shelf and keep
its remaining logic with the facts owner and the shape matcher that actually own
those responsibilities.

This is BoxShape-only cleanup. It does not change accepted scan-loop shapes or
recipe behavior.

## Boundaries

- Keep `release_enabled()` with the facts owner that gates extraction.
- Keep local-declaration and tail-step matching helpers inside the shape-route
  owner.
- Do not alter recipe-building or route semantics.

## Result

- Deleted `facts/loop_collect_using_entries_v0_helpers.rs`.
- Moved `release_enabled()` into `loop_collect_using_entries_v0.rs`.
- Moved the remaining shape-local helpers into
  `loop_collect_using_entries_v0_shape_routes.rs`.
- Removed the dead module declaration from `facts/mod.rs`.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
git diff --check
tools/checks/dev_gate.sh quick
```
