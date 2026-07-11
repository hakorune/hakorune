---
Status: Landed
Date: 2026-04-29
Scope: rustfmt drift closeout
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/box_factory/registry.rs
  - src/grammar/generated.rs
  - src/mir/builder/control_flow/plan/canon/generic_loop/step/placement.rs
  - src/mir/builder/control_flow/plan/composer/coreloop_gates.rs
  - src/mir/builder/control_flow/plan/condition_env_builder.rs
  - src/mir/builder/control_flow/plan/facts/loop_tests_parts/planner_ctx.rs
  - src/mir/builder/control_flow/plan/normalizer/simple_while_coreloop_builder.rs
  - src/mir/builder/control_flow/plan/plan_build_session.rs
  - src/mir/builder/control_flow/plan/planner/outcome.rs
  - src/using/resolver.rs
---

# 291x-712: Rustfmt Drift Closeout

## Why

After `291x-711`, compiler warning cleanup was warning-free, but
`cargo fmt --check` still found pre-existing formatting drift across a small set
of Rust files.

This is not structural compiler vocabulary work, so keep it as a separate style
closeout slice.

## Decision

Apply `cargo fmt` once and update the current pointer to make the formatting
baseline explicit.

No route, planner, lowering, parser, or resolver behavior is intentionally
changed in this card.

## Changes

- applied rustfmt to the existing drift set
- kept the generated grammar file formatting aligned with repository rustfmt
- updated `CURRENT_STATE.toml` to point at this format closeout card

## Result

- `cargo fmt --check` is clean
- the warning-cleanup baseline from `291x-711` remains unchanged

## Proof

```bash
cargo fmt --check
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
