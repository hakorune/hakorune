---
Status: Landed
Date: 2026-04-28
Scope: delete zero-use plan-side SSA exit-binding wrappers
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/ssa/exit_binding.rs
  - src/mir/builder/control_flow/ssa/exit_binding_applicator.rs
  - src/mir/builder/control_flow/ssa/exit_binding_constructor.rs
  - src/mir/builder/control_flow/ssa/exit_binding_validator.rs
  - src/mir/builder/control_flow/plan/mod.rs
---

# 291x-581: SSA Exit-Binding Plan Wrapper Prune

## Goal

Delete the plan-side wrappers for SSA exit-binding helpers now that ownership is
fully local to `control_flow::ssa` and no plan-side callers remain.

This is a BoxShape-only deletion card. It does not change SSA behavior.

## Evidence

The four plan-side files were pure re-exports:

- `plan/exit_binding.rs`
- `plan/exit_binding_applicator.rs`
- `plan/exit_binding_constructor.rs`
- `plan/exit_binding_validator.rs`

Repository search found no live `plan::*exit_binding*` callers, so there was no
owner-path migration left to perform.

## Boundaries

- Delete only zero-use wrapper files and their module declarations.
- Do not modify the real SSA owner implementations.
- Leave later boundary-review cards untouched.

## Acceptance

- No `plan::exit_binding*` wrapper files remain.
- No `control_flow::plan::exit_binding*` references remain in `src/` or `tests/`.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Removed the four zero-use plan-side SSA wrapper files.
- Removed their dead module declarations from `plan/mod.rs`.
- Kept the real implementations under `control_flow::ssa`.

## Verification

```bash
rg -n "control_flow::plan::exit_binding|plan::exit_binding" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
