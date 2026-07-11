---
Status: Landed
Date: 2026-04-29
Scope: route-prep dead shelf cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/compiler-task-map-ssot.md
  - docs/development/current/main/design/plan-mod-layout-ssot.md
  - src/mir/builder/control_flow/mod.rs
  - src/mir/builder/control_flow/joinir/route_entry/mod.rs
  - src/mir/builder/control_flow/plan/common_init.rs
  - src/mir/builder/control_flow/plan/loop_true_counter_extractor.rs
  - src/mir/builder/control_flow/plan/loop_scope_shape_builder.rs
  - src/mir/builder/control_flow/plan/mod.rs
  - src/mir/builder/control_flow/plan/route_prep_pipeline.rs
  - src/mir/builder/control_flow/utils.rs
  - src/mir/join_ir/lowering/loop_scope_shape/builder.rs
  - src/mir/join_ir/lowering/loop_scope_shape/shape.rs
---

# 291x-713: Route-Prep Pipeline Shelf Prune

## Why

`route_prep_pipeline.rs` still carried a private `RoutePrepContext` /
`RouteVariant` scaffold and several local `#[allow(dead_code)]` markers.

The module had no live caller outside its own tests. Current route planning now
enters through Facts, Recipe, Composer, and dedicated route owners, so this file
was a dead shelf rather than structural vocabulary.

Deleting it exposed the same shelf boundary on its private support modules:
`common_init`, `loop_true_counter_extractor`, `loop_scope_shape_builder`, and
`control_flow::utils`.

## Decision

Delete the unused route-prep scaffold instead of retaining it as a dormant
future hook.

Future shared preprocessing must be introduced through the active owner layer
that consumes it. Do not revive `route_prep_pipeline.rs` as a second
preprocessing SSOT.

## Changes

- removed `src/mir/builder/control_flow/plan/route_prep_pipeline.rs`
- removed the now-unowned `common_init`, `loop_true_counter_extractor`, and
  `loop_scope_shape_builder` / `control_flow::utils` helper shelves
- removed the private module declaration from `plan/mod.rs`
- updated current layout/task-map docs so the deleted file is not listed as an
  active module

## Result

- one dead private scaffold, its support shelves, and their local warning
  markers are gone
- active route planning ownership stays with Facts / Recipe / Composer

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
