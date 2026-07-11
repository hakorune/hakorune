---
Status: Landed
Date: 2026-04-29
Scope: lib-test warning shelf cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/normalizer/README.md
  - src/mir/builder/control_flow/plan/REGISTRY.md
  - src/mir/builder/control_flow/plan/normalizer/mod.rs
  - src/mir/builder/control_flow/plan/trim_validator.rs
  - src/mir/builder/control_flow/plan/route_prep_pipeline.rs
  - src/mir/join_ir/lowering/common/condition_only_emitter.rs
---

# 291x-711: Test-Target Warning Shelf Prune

## Why

After `291x-710`, release lib warnings were zero, but `cargo test --lib --no-run`
still reported four `lib test` warnings:

- `normalizer/loop_break.rs::LoopBreakPlan`
- `PlanNormalizer::normalize_loop_break`
- `TrimValidator::emit_whitespace_check`
- `route_prep_pipeline::tests::test_condition`

These were not active structural vocabulary. They were test-target shelves with
no live owner path.

## Decision

Prune the dead test-target shelves and keep the current SSOT pointers aligned.

Break-route re-expansion must not recreate a test-only normalizer shelf. It
should enter through `ExitMap` / feature pipeline vocabulary as a new focused
box.

## Changes

- removed the unused test-only `normalizer/loop_break.rs` module
- removed the unused `TrimValidator::emit_whitespace_check` helper
- removed the unused `route_prep_pipeline::tests::test_condition` helper
- updated current normalizer README / registry / design pointers that still
  treated `normalizer/loop_break.rs` as current
- updated the ConditionOnlyEmitter comment to describe the inline whitespace
  check directly

## Result

- `cargo test --lib --no-run` warning count moved from **4** to **0**
- no release route, recipe, or lower behavior changed

## Proof

```bash
cargo test --lib --no-run
```
