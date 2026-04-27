---
Status: Landed
Date: 2026-04-28
Scope: prune zero-use plan/facts compatibility wrappers
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/facts/expr_value.rs
  - src/mir/builder/control_flow/facts/block_policies.rs
  - src/mir/builder/control_flow/plan/facts/mod.rs
---

# 291x-577: Unused Plan Facts Wrapper Prune

## Goal

Delete zero-use `plan/facts` compatibility wrappers before the broader
owner-path migration cards.

This card only removes dead compatibility shelves. It does not move live
callers or change any facts logic.

## Evidence

The residue inventory pointed at `expr_value` and `block_policies` as the first
small wrappers to verify.

Repository search showed no live callers for either plan-side path:

```text
control_flow::plan::facts::expr_value
control_flow::plan::facts::block_policies
```

The only remaining references were:

- the wrapper files themselves
- the module declarations in `plan/facts/mod.rs`

## Cleaner Boundary

```text
facts/expr_value.rs
  owns value-expression checks

facts/block_policies.rs
  owns block-policy predicates

plan/facts/*
  keeps only live plan-owned facts modules
```

## Boundaries

- Delete only zero-use compatibility wrappers.
- Do not move or rewrite live callers in this card.
- Leave broader `plan::facts` owner-path migration work to `291x-578+`.

## Acceptance

- No `plan::facts::expr_value` references remain.
- No `plan::facts::block_policies` references remain.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Removed `plan/facts/expr_value.rs`.
- Removed `plan/facts/block_policies.rs`.
- Removed the corresponding dead module declarations from `plan/facts/mod.rs`.
- Advanced `CURRENT_STATE.toml` to the next queue item after the zero-use prune.

## Verification

```bash
rg -n "control_flow::plan::facts::expr_value|plan::facts::expr_value|control_flow::plan::facts::block_policies|plan::facts::block_policies" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
