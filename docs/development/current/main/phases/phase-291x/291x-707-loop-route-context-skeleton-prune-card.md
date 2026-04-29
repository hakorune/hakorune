---
Status: Landed
Date: 2026-04-29
Scope: route-entry context cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
---

# 291x-707: LoopRouteContext Skeleton Prune

## Why

The `LoopRouteContext` struct still carried a `skeleton` slot from an older
canonicalizer handoff idea, but route-entry owner-path checks showed that the field
was no longer used:

- no route-entry/planner/lowerer code read `ctx.skeleton`
- `LoopRouteContext::new()` always initialized it to `None`
- no production or test construction sites ever populated it afterward

That made the field pure route-context baggage in lib builds.

## Changes

- removed the unused `LoopRouteContext::skeleton` field
- dropped the now-dead `LoopSkeleton` import from the route-entry router
- kept the rest of the route-entry context surface unchanged

## Result

- `cargo build --release` warning count moved from **8** to **7**
- nested/single-entry route tests still use the same context constructor without the
  stale canonicalizer slot

## Proof

```bash
cargo build --release
cargo test --release --lib single_entry_prefers_nested_path -- --nocapture
```
