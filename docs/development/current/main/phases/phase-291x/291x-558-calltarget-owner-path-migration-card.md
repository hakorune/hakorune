---
Status: Landed
Date: 2026-04-28
Scope: Move CallTarget users off builder_calls compatibility paths
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/tools/check-scripts-index.md
  - src/mir/builder.rs
  - src/mir/builder/builder_calls.rs
  - src/mir/builder/calls/call_target.rs
  - tools/checks/dev_gate.sh
  - tools/checks/mir_builder_calltarget_owner_guard.sh
---

# 291x-558: CallTarget Owner Path Migration

## Goal

Move active `CallTarget` users off the `builder_calls` compatibility shell.

`CallTarget` is owned by `src/mir/builder/calls/call_target.rs`. The
compatibility shell `src/mir/builder/builder_calls.rs` should not re-export
active call vocabulary after the call-system split.

## Inventory

Removed compatibility re-export:

- `src/mir/builder/builder_calls.rs`
  - `pub use super::calls::call_target::CallTarget`

Updated owner path:

- `src/mir/builder.rs`
  - `pub(crate) use calls::CallTarget`

Migrated callers:

- `src/mir/builder/builder_build.rs`
- `src/mir/builder/exprs_call.rs`
- `src/mir/builder/method_call_handlers.rs`
- `src/mir/builder/ops/arithmetic.rs`
- `src/mir/builder/ops/comparison.rs`
- `src/mir/builder/ops/unary.rs`
- `src/mir/builder/rewrite/known.rs`
- `src/mir/builder/stmts/print_stmt.rs`
- `src/mir/builder/utils/boxcall_emit.rs`

Guarded regrowth:

- `tools/checks/mir_builder_calltarget_owner_guard.sh`

## Cleaner Boundary

```text
builder/calls/call_target.rs
  owns CallTarget vocabulary

builder.rs
  exposes crate-local builder facade import

builder_calls.rs
  remains only for old MirBuilder helper shims, not active call vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change call lowering behavior.
- Do not rename `CallTarget`.
- Do not remove `builder_calls.rs` helper methods.
- Do not touch CoreMethodContract/CoreOp or `.inc` lanes.

## Acceptance

- No `builder_calls::CallTarget` callers remain.
- `builder_calls.rs` no longer re-exports `CallTarget`.
- `bash tools/checks/mir_builder_calltarget_owner_guard.sh` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Repointed builder CallTarget users to the current owner path.
- Removed the compatibility re-export from `builder_calls.rs`.
- Added a quick dev-gate guard for CallTarget owner-path hygiene.

## Verification

```bash
rg -n "builder_calls::CallTarget|pub use super::calls::call_target::CallTarget" src/mir/builder -g'*.rs'
bash tools/checks/mir_builder_calltarget_owner_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
