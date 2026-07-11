---
Status: Landed
Date: 2026-04-27
Scope: Prune RC insertion pass re-exports from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/passes/rc_insertion.rs
---

# 291x-539: RC Insertion Root Export Prune

## Goal

Keep the RC insertion pass on its owner path under `mir::passes` instead of
re-exporting pass functions and pass-local statistics through the broad MIR
root facade.

The MIR root facade should not become a convenience barrel for pass-specific
entry points unless that entry point is part of durable compiler orchestration.

## Inventory

Removed root exports:

- `insert_rc_instructions`
- `RcInsertionStats`

Existing consumers already use owner paths:

- `src/mir/compiler/mod.rs`
- `src/bin/rc_insertion_selfcheck/helpers.rs`

## Cleaner Boundary

```text
mir::passes::rc_insertion
  owns RC insertion pass API and statistics

mir root
  does not re-export pass-local RC insertion vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change RC insertion behavior.
- Do not change compiler pass order.
- Do not change feature flags or selfcheck behavior.

## Acceptance

- MIR root no longer re-exports `insert_rc_instructions` or
  `RcInsertionStats`.
- Existing consumers continue to use `mir::passes::rc_insertion`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed pass-local RC insertion API from the MIR root export surface.
- Preserved RC insertion behavior and selfcheck import paths.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
