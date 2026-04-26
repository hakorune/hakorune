---
Status: Landed
Date: 2026-04-26
Scope: JoinIR route detector support facade physical owner inventory
Related:
  - src/mir/loop_route_detection/mod.rs
  - src/mir/loop_route_detection/support/mod.rs
  - src/mir/loop_route_detection/support/README.md
  - src/mir/loop_route_detection/legacy/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-382-joinir-route-detector-compatibility-module-ownership-closeout-card.md
---

# 291x-383: JoinIR Route Detector Support Facade Physical Owner Inventory

## Goal

Inventory whether the `loop_route_detection::support` facades should become
physical owner modules next.

This is BoxShape-only. Do not move files in this card.

## Current Surface

The parent module surface is now:

```text
classify
LoopFeatures
LoopRouteKind
support
```

`legacy/` is private implementation storage. No source/test caller uses the old
parent compatibility paths:

```text
loop_route_detection::break_condition_analyzer
loop_route_detection::function_scope_capture
loop_route_detection::loop_body_carrier_promoter
loop_route_detection::loop_body_cond_promoter
loop_route_detection::loop_condition_scope
loop_route_detection::mutable_accumulator_analyzer
loop_route_detection::pinned_local_analyzer
loop_route_detection::trim_loop_helper
```

## Facade Inventory

| Support facade | Current storage | Move risk | Suggested action |
| --- | --- | --- | --- |
| `support::break_condition` | `legacy/break_condition_analyzer.rs` | low | move first |
| `support::locals::pinned` | `legacy/pinned_local_analyzer.rs` | low | move first |
| `support::locals::mutable_accumulator` | `legacy/mutable_accumulator_analyzer.rs` | low | move first |
| `support::trim` | `legacy/trim_loop_helper.rs` | medium | move after small family |
| `support::function_scope` | `legacy/function_scope_capture/` | medium | move as directory series |
| `support::condition_scope` | `legacy/loop_condition_scope.rs` + `condition_var_analyzer.rs` | medium | move together |
| `support::body_local::carrier` | `legacy/loop_body_carrier_promoter.rs` + trim detector dependency | high | move after trim |
| `support::body_local::condition` | `legacy/loop_body_cond_promoter.rs` + digitpos dependency | high | move after condition scope |

## Decision

Do not do one large physical move.

Use a short Refactor Series Mode:

```text
1. Update legacy/support READMEs to reflect current boundary.
2. Add a guard preventing parent legacy-surface regrowth.
3. Move low-risk support families first.
4. Move medium/high families only after each family has a focused cargo check.
```

## Next Cleanup

Align docs before moving files:

```text
src/mir/loop_route_detection/legacy/README.md
src/mir/loop_route_detection/support/README.md
```

Then add a guard for:

```text
pub mod legacy
pub use legacy::
loop_route_detection::<old compatibility module>
```

## Non-Goals

- No physical file move.
- No support facade deletion.
- No route classifier API change.
- No route lowerer behavior change.

## Validation

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
