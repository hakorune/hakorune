---
Status: Landed
Date: 2026-04-28
Scope: centralize duplicated scan-family final-values binding helpers
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/parts/loop_/final_values.rs
  - src/mir/builder/control_flow/plan/parts/loop_/mod.rs
  - src/mir/builder/control_flow/plan/parts/entry.rs
  - src/mir/builder/control_flow/plan/loop_scan_v0
  - src/mir/builder/control_flow/plan/loop_scan_methods_v0
  - src/mir/builder/control_flow/plan/loop_scan_methods_block_v0
  - src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0
---

# 291x-606: Scan Final-Values Helper Owner

## Goal

Move the duplicated `apply_loop_final_values_to_bindings` helper out of
scan-family v0 modules and into the neutral loop parts owner.

This is BoxShape-only cleanup. It does not change segment routing, nested-loop
handoff, final value semantics, accepted shapes, or lowering behavior.

## Boundaries

- Keep scan-family route logic in the existing scan modules.
- Keep the helper body unchanged while moving it to `plan/parts/loop_`.
- Do not merge scan-family pipelines or change recipe contracts in this card.

## Result

- Added `parts/loop_/final_values.rs` as the single owner for applying loop
  final values to `variable_map` and existing current bindings.
- Re-exported the helper through `parts::entry`.
- Migrated scan-family callers to the shared helper.
- Deleted the empty `helpers.rs` shelves from `loop_scan_v0`,
  `loop_scan_methods_v0`, and `loop_scan_phi_vars_v0`.
- Removed duplicated local helper definitions from
  `loop_scan_methods_block_v0`.

## Verification

```bash
! rg -n "fn apply_loop_final_values_to_bindings|mod helpers;|super::helpers" src/mir/builder/control_flow/plan/loop_scan_v0 src/mir/builder/control_flow/plan/loop_scan_methods_v0 src/mir/builder/control_flow/plan/loop_scan_methods_block_v0 src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0
cargo test -q loop_scan
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
