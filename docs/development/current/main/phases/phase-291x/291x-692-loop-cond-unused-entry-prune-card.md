---
Status: Landed
Date: 2026-04-29
Scope: loop-cond cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/features/loop_cond_bc_else_patterns/guard_break.rs
  - src/mir/builder/control_flow/plan/parts/conditional_update.rs
  - src/mir/builder/control_flow/plan/generic_loop/body_check_tests.rs
---

# 291x-692: Loop-Cond Unused Entry Prune

## Why

The warning-backlog inventory in `291x-691` identified a small loop-cond shelf
whose exported entrypoints no longer had any owner-path callers. This card keeps
the cleanup narrow: remove only dead forwarding/convenience entrypoints and fix
the one test import drift surfaced while validating the card.

## Changes

- removed the dead convenience wrapper
  `loop_cond_bc_else_patterns::guard_break::lower_else_guard_break_if`
- removed dead conditional-update convenience entrypoints that were superseded
  by the recipe-first and direct `try_lower_*` owners:
  - `lower_conditional_update_if_assume`
  - `lower_conditional_update_if_assume_with_break_phi_args`
  - `try_lower_general_if_adapter`
- fixed the stale `body_check_tests.rs` import so the overlap test follows the
  current `shape_detection::resolve_v1_shape_matches` owner path

## Result

- `cargo build --release` warning count moved from **48** to **44**
- the loop-cond / conditional-update surface is thinner without changing live
  lowering paths

## Proof

```bash
cargo build --release
cargo test --release recipe_authority_allows_general_if_lowering_in_release -- --nocapture
cargo test --release generic_loop_v1_shape_overlap_freezes -- --nocapture
```
