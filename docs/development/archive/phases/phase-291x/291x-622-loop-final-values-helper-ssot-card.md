---
Status: Landed
Date: 2026-04-28
Scope: prune duplicate loop final_values binding helpers and keep parts::loop_ as the SSOT
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/parts/loop_/final_values.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_bc_nested_carriers.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_bc_item_stmt.rs
  - src/mir/builder/control_flow/plan/features/generic_loop_body/helpers.rs
  - src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs
---

# 291x-622: Loop Final Values Helper SSOT

## Goal

Remove duplicate `apply_loop_final_values_to_bindings` helper bodies from
feature-local modules and keep the loop parts helper as the single owner.

This is BoxShape-only cleanup. It does not change loop lowering, nested-loop
acceptance, PHI construction, carrier propagation, or final_values semantics.

## Evidence

`parts::loop_::final_values` already owned the shared helper and most scan-family
callers imported it through `parts::entry`.

Two feature-local copies still repeated the same implementation:

- `features/loop_cond_bc_nested_carriers.rs`
- `features/generic_loop_body/helpers.rs`

Both copies inserted each `CorePlan::Loop.final_values` entry into
`builder.variable_ctx.variable_map` and refreshed the current binding map only
when that name already existed there.

## Boundaries

- Keep `parts::loop_::final_values` as the helper implementation.
- Migrate feature callers to import `parts::entry::apply_loop_final_values_to_bindings`.
- Do not change `extend_nested_loop_carriers`.
- Do not change loop plan construction or recipe dispatch.
- Do not add a compatibility re-export in the feature modules.

## Acceptance

- No feature-local `apply_loop_final_values_to_bindings` definitions remain.
- `cargo test -q loop_cond_bc` passes.
- `cargo test -q generic_loop_body` passes.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed duplicate helper bodies from loop-cond/break/continue and generic-loop
  feature modules.
- Updated feature callers to use the loop parts SSOT directly.
- Kept final_values propagation behavior unchanged.

## Verification

```bash
rg -n "pub\\(super\\) fn apply_loop_final_values_to_bindings|loop_cond_bc_nested_carriers::apply_loop_final_values_to_bindings" src/mir/builder/control_flow/plan -g'*.rs'
cargo test -q loop_cond_bc
cargo test -q generic_loop_body
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
