---
Status: Landed
Date: 2026-04-28
Scope: extract duplicated methods-scan guard predicates into the shared scan predicate owner
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/scan_common_predicates.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0_helpers.rs
  - src/mir/builder/control_flow/facts/loop_scan_methods_block_v0_shape_routes.rs
---

# 291x-598: Methods Guard Predicate Extract

## Goal

Move duplicated methods-scan tail/guard predicates into
`facts::scan_common_predicates` so both the plain and block scan-methods facts
families read the same low-level AST matcher vocabulary.

This is BoxShape-only cleanup. It does not change accepted shapes.

## Boundaries

- Share only the duplicated low-level AST matchers.
- Keep block-only statement walkers and segment builders in the
  `loop_scan_methods_block_v0_helpers` owner.
- Do not change route-local reject strings or recipe-builder behavior.

## Result

- Added shared `match_next_i_guard()` to `scan_common_predicates.rs`.
- Reused the existing shared `extract_step_var_from_tail()` for methods-scan tail
  matching.
- Removed duplicated local copies from:
  - `loop_scan_methods_v0.rs`
  - `loop_scan_methods_block_v0_helpers.rs`
- Updated block shape routes to import the shared predicates directly.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
git diff --check
tools/checks/dev_gate.sh quick
```
