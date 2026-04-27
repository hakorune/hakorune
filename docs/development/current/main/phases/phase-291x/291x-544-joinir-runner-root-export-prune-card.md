---
Status: Landed
Date: 2026-04-27
Scope: Prune JoinIR runner convenience re-exports from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/join_ir_runner/mod.rs
---

# 291x-544: JoinIR Runner Root Export Prune

## Goal

Keep direct JoinIR runner APIs on `mir::join_ir_runner` instead of
re-exporting them through the broad MIR root facade.

The direct JoinIR runner is a development harness / structure validation route.
It is not core MIR model vocabulary and should not be part of the root facade
unless a durable public façade need appears.

## Inventory

Removed root exports:

- `run_joinir_function`
- `JoinRuntimeError`
- `JoinValue`

Current root-path consumers:

- None found.

Existing consumers use owner modules:

- `crate::mir::join_ir_runner::{run_joinir_function, JoinValue}`
- `crate::mir::join_ir_ops::JoinValue`

## Cleaner Boundary

```text
mir::join_ir_runner
  owns direct JoinIR runner harness API

mir root
  does not re-export runner harness convenience API
```

## Boundaries

- BoxShape-only.
- Do not change JoinIR runner behavior.
- Do not change JoinIR op value conversion behavior.
- Do not change Route A / Route B test policy.

## Acceptance

- MIR root no longer re-exports direct JoinIR runner APIs.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed direct runner convenience APIs from the MIR root export surface.
- Preserved owner-module runner APIs.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
