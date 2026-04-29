# 291x-727 LoopForm Route Router Shelf Prune Card

Status: Landed
Date: 2026-04-29
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/loop_route_router.rs`
- `src/mir/join_ir/lowering/loop_routes/*`
- `src/mir/join_ir/lowering/if_lowering_router.rs`
- `src/mir/join_ir/README.md`
- `src/mir/loop_route_detection/mod.rs`
- `src/mir/loop_route_detection/features.rs`
- `tools/test_phase188_foundation.sh`

## Why

The LoopForm route router was still exported, but no live caller used it. Its
remaining route modules were compatibility stubs or placeholder scaffolding that
returned `None`, so keeping the entrypoint only preserved a dead fallback shelf.

`tools/test_phase188_foundation.sh` was also unreferenced and checked removed
Phase 188 scaffolding such as `loop_patterns.rs`, so it no longer represented a
valid gate.

## Decision

Remove the unused LoopForm route-router surface. Live loop route lowering remains
owned by the plan/recipe path plus the still-live direct lowerers. Several old
direct lowerer shelves were retired later (`loop_with_break_minimal` in
`291x-733`, scan/split shelves in `291x-734`, and `loop_with_if_phi_if_sum` in
`291x-735`).

## Changes

- Removed `loop_route_router.rs` and the `try_lower_loop_route_to_joinir`
  re-export.
- Removed the remaining `loop_routes` compatibility modules.
- Updated local lowering docs/README references to stop pointing at the removed
  router.
- Removed the stale Phase 188 foundation script.
- Removed the now-unused LoopForm-to-`LoopFeatures` extractor re-export and
  helper.

## Proof

- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `rg -n "try_lower_loop_route_to_joinir|loop_route_router|lower_simple_while_to_joinir|lower_loop_with_continue_to_joinir|lower_nested_loop_minimal_to_joinir|src/mir/join_ir/lowering/loop_routes|test_phase188_foundation" src tests tools docs/development/current/main/CURRENT_STATE.toml src/mir/join_ir/README.md -g '*.rs' -g '*.sh' -g '*.py' -g '*.md' -g '*.toml'`
- `rg -n "loop_route_detection::extract_features|pub\\(crate\\) use features::extract_features|fn extract_features\\(loop_form|LoopForm.*extract_features" src tests docs/development/current/main/CURRENT_STATE.toml src/mir/join_ir/README.md -g '*.rs' -g '*.md' -g '*.toml'`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
