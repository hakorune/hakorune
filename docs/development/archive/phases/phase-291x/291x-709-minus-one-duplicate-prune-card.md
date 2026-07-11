---
Status: Landed
Date: 2026-04-29
Scope: generic-loop helper cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/generic_loop/body_check_shape_detectors/utils.rs
  - src/mir/builder/control_flow/plan/generic_loop/body_check_extractors.rs
---

# 291x-709: Minus-One Duplicate Prune

## Why

The remaining `is_loop_var_minus_one` warning was coming from
`body_check_shape_detectors::utils`, but the live owner-path had already moved to
`body_check_extractors::is_loop_var_minus_one`:

- `body_check::expr_matchers::call` imports the extractor helper
- no production code referenced the old duplicate in `utils.rs`

That left the `utils.rs` copy as dead baggage.

## Changes

- removed the dead duplicate `is_loop_var_minus_one()` from
  `generic_loop/body_check_shape_detectors/utils.rs`
- kept the live extractor-owned helper unchanged

## Result

- `cargo build --release` warning count moved from **5** to **4**
- the remaining lib-warning backlog is now fully concentrated in structural facts/enums
  rather than duplicate helper shelves

## Proof

```bash
cargo build --release
cargo test --release --lib generic_loop_v0_allows_loop_var_from_add_expr_in_condition -- --nocapture
```
