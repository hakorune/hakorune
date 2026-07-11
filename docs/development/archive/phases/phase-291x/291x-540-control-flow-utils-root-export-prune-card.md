---
Status: Landed
Date: 2026-04-27
Scope: Prune control-flow utility re-exports from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/utils/mod.rs
---

# 291x-540: Control-Flow Utils Root Export Prune

## Goal

Keep builder control-flow helper utilities on `mir::utils` instead of
re-exporting them through the broad MIR root facade.

These helpers are implementation support for MIR builder/control-flow code, not
core MIR model vocabulary and not durable compiler façade entry points.

## Inventory

Removed root exports:

- `capture_actual_predecessor_and_jump`
- `collect_phi_incoming_if_reachable`
- `execute_statement_with_termination_check`
- `is_current_block_terminated`

Existing production consumer already imports the owner path:

- `src/mir/builder/stmts/block_stmt.rs`

## Cleaner Boundary

```text
mir::utils
  owns builder control-flow helper utilities

mir root
  does not re-export implementation helper utilities
```

## Boundaries

- BoxShape-only.
- Do not change helper behavior.
- Do not change MIR builder control-flow behavior.
- Do not change PHI incoming collection or termination checks.

## Acceptance

- MIR root no longer re-exports control-flow utility helpers.
- Existing consumers continue to use `mir::utils`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed implementation-helper utilities from the MIR root export surface.
- Preserved control-flow builder behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
