---
Status: Landed
Date: 2026-04-29
Scope: planner context cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/planner/context.rs
  - src/mir/builder/control_flow/plan/single_planner/rules.rs
  - src/mir/builder/control_flow/plan/facts/loop_tests_parts/planner_ctx.rs
---

# 291x-706: PlannerContext Baggage Prune

## Why

Direct owner-path checks showed that `PlannerContext` was only being threaded through
the planner API and test fixtures. The fields themselves were never read:

- `route_kind`
- `in_static_box`
- `debug`

`build_plan_with_facts_ctx()` already discarded the context value, and
`try_build_loop_facts_with_ctx()` accepted `_ctx` without consuming it, so the struct
had become pure baggage in lib builds.

## Changes

- collapsed `PlannerContext` to an empty struct
- removed dead field population from single-planner rule setup
- updated planner-context fixtures to construct the now-empty context directly

## Result

- `cargo build --release` dropped the `PlannerContext` warning and exposed the next
  unread baggage field (`LoopRouteContext::skeleton`), so the overall lib-warning
  count stayed at **8** while the backlog shifted forward
- planner/facts tests still exercise the ctx-taking API surface without carrying dead
  metadata fields

## Proof

```bash
cargo build --release
cargo test --release --lib loopfacts_ctx_keeps_simple_while_route_even_when_kind_mismatch -- --nocapture
cargo test --release --lib loopfacts_ctx_allows_bool_predicate_scan_route_in_static_box -- --nocapture
```
