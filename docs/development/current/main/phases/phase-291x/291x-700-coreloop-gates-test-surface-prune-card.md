---
Status: Landed
Date: 2026-04-29
Scope: composer gate cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/composer/coreloop_gates.rs
  - src/mir/builder/control_flow/plan/composer/coreloop_v1.rs
  - src/mir/builder/control_flow/plan/composer/coreloop_single_entry.rs
  - src/mir/builder/control_flow/plan/composer/coreloop_v2_nested_minimal.rs
---

# 291x-700: Coreloop Gates Test-Surface Prune

## Why

`coreloop_gates.rs` mixed two different surfaces:

- live nested-minimal gates used by non-test code
- v1/single-entry helpers that are only reached from `#[cfg(test)]` modules

That made lib builds report dead-code warnings for helpers that still need to
exist for focused composer tests.

## Changes

- gated `ExitKindFacts` import behind `#[cfg(test)]`
- marked the v1/single-entry-only helpers as `#[cfg(test)]`
- kept `coreloop_base_gate()` and `exit_kinds_empty()` live for nested-minimal
  production code

## Result

- `cargo build --release` warning count moved from **25** to **20**
- lib builds now expose only the live coreloop gate surface while test builds
  still cover the v1/single-entry helper paths

## Proof

```bash
cargo build --release
cargo test --release --lib coreloop_v1_rejects_loop_break_with_cleanup -- --nocapture
cargo test --release --lib single_entry_prefers_nested_path -- --nocapture
```
