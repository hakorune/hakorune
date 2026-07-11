---
Status: Landed
Date: 2026-04-28
Scope: Remove builder_calls compatibility shell after call-system owner migration
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/tools/check-scripts-index.md
  - src/mir/builder.rs
  - src/mir/builder/calls/utils.rs
  - src/mir/builder/decls.rs
  - src/mir/builder/exprs.rs
  - tools/checks/mir_builder_calltarget_owner_guard.sh
---

# 291x-560: Builder Calls Shell Closeout

## Goal

Delete the remaining `builder_calls.rs` compatibility shell.

After `291x-558`, the file only forwarded two `MirBuilder` associated helpers
to `calls/utils.rs`. The owner module can hold those helpers directly, letting
the builder module map drop the old compatibility barrel.

## Inventory

Moved associated helpers:

- `MirBuilder::parse_type_name_to_mir`
- `MirBuilder::extract_string_literal`

New owner:

- `src/mir/builder/calls/utils.rs`

Removed compatibility shell:

- `src/mir/builder/builder_calls.rs`
- `mod builder_calls;`

Cleaned stale comments:

- `src/mir/builder/decls.rs`
- `src/mir/builder/exprs.rs`
- `src/mir/builder/calls/annotation.rs`

Guarded regrowth:

- `tools/checks/mir_builder_calltarget_owner_guard.sh`

## Cleaner Boundary

```text
builder/calls/utils.rs
  owns call utility functions and MirBuilder utility shims

builder.rs
  no builder_calls module

builder_calls.rs
  deleted; compatibility shell must not regrow
```

## Boundaries

- BoxShape-only.
- Do not change call lowering behavior.
- Do not change helper names or visibility to builder descendants.
- Do not touch CoreMethodContract/CoreOp or `.inc` lanes.

## Acceptance

- No `builder_calls` live Rust references remain.
- `src/mir/builder/builder_calls.rs` is absent.
- `bash tools/checks/mir_builder_calltarget_owner_guard.sh` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Moved the two remaining associated helper shims into `calls/utils.rs`.
- Deleted `builder_calls.rs` and removed it from the builder module map.
- Extended the CallTarget owner guard so the compatibility shell cannot regrow.

## Verification

```bash
rg -n "builder_calls" src/mir/builder -g'*.rs'
bash tools/checks/mir_builder_calltarget_owner_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
