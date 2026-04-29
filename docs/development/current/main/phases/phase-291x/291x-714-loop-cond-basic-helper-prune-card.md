---
Status: Landed
Date: 2026-04-29
Scope: loop-cond helper shelf cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/loop_cond_unified_helpers.rs
---

# 291x-714: Loop-Cond Basic Helper Prune

## Why

`loop_cond_unified_helpers::count_control_flow_basic` was an unused alternate
entry beside the active `count_control_flow_with_returns` helper.

The active loop-cond facts call the returns-enabled counter. The basic counter
had no owner path and only existed behind `#[allow(dead_code)]`.

## Decision

Remove the unused alternate helper instead of holding a second control-flow
counting entry.

## Changes

- removed `count_control_flow_basic`
- left `count_control_flow_with_returns` as the single loop-cond counter entry

## Result

- plan-side non-structural dead-code shelf is smaller
- loop-cond behavior is unchanged

## Proof

```bash
cargo test --lib --no-run
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
```
