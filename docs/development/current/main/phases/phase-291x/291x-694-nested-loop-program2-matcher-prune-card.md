---
Status: Landed
Date: 2026-04-29
Scope: generic-loop cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/generic_loop/body_check_shape_detectors/nested_loop_program2.rs
  - src/mir/builder/control_flow/plan/generic_loop/body_check_tests.rs
---

# 291x-694: Nested Loop Program2 Matcher Prune

## Why

The warning backlog still contained five simplified `nested_loop_program2`
matcher entrypoints with no owner-path callers. The active shape detection path
already routes through the `_shape(...)` variants, so keeping the simplified
helpers only preserved dead warning surface.

## Changes

- removed dead simplified matchers from
  `generic_loop/body_check_shape_detectors/nested_loop_program2.rs`
  - `matches_parse_program2_nested_loop_if_else_return_literal`
  - `matches_parse_program2_nested_loop_if_else_return_var_local`
  - `matches_parse_program2_nested_loop_if_else_if_return`
  - `matches_parse_program2_nested_loop_if_else_if_else_return`
  - `matches_parse_program2_nested_loop_if_else_return_literal_local`
- trimmed the now-unused expr-matcher imports in the same file

## Result

- `cargo build --release` warning count moved from **41** to **36**
- the Program2 nested-loop detector now exposes only the active `_shape`
  owner-path helpers

## Proof

```bash
cargo build --release
cargo test --release generic_loop_v1_shape_overlap_freezes -- --nocapture
```
