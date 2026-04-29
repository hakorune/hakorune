---
Status: Landed
Date: 2026-04-29
Scope: test-only helper cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/normalizer/mod.rs
  - src/mir/builder/control_flow/plan/features/edgecfg_stubs.rs
---

# 291x-708: Test-Only Normalizer Helper Prune

## Why

Owner-path checks showed that two remaining lib warnings came from helper surfaces that
were already effectively test-only:

- `simple_while_coreloop_builder` was only reachable through `coreloop_v0`, and the
  entire `coreloop_single_entry` / `coreloop_v0` / `coreloop_v1` composer shelf is
  already compiled under `#[cfg(test)]`
- `build_loop_cond_branch()` was only used by the test-only `normalizer::loop_break`
  harness and the `value_join_demo_if2` test module

Keeping those helpers live in lib builds only added dead-code noise.

## Changes

- gated `normalizer::simple_while_coreloop_builder` behind `#[cfg(test)]`
- gated `edgecfg_stubs::build_loop_cond_branch()` behind `#[cfg(test)]`
- kept the existing test harnesses and demo tests unchanged

## Result

- `cargo build --release` warning count moved from **7** to **5**
- lib builds now keep only the remaining structural facts/enums plus the dead duplicate
  `is_loop_var_minus_one` helper in this warning lane

## Proof

```bash
cargo build --release
cargo test --release --lib build_simple_while_coreloop_has_expected_frag_shape -- --nocapture
cargo test --release --lib demo_if2_valuejoin_emits_phi_and_return -- --nocapture
```
