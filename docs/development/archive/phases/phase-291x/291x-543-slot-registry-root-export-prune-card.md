---
Status: Landed
Date: 2026-04-27
Scope: Prune slot-registry type aliases from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - src/mir/slot_registry.rs
---

# 291x-543: Slot-Registry Root Export Prune

## Goal

Keep slot-registry type aliases on `mir::slot_registry` instead of exporting
them through the broad MIR root facade.

`BoxTypeId` and `MethodSlot` are registry-local vocabulary, not core MIR model
types. Consumers that need slot-registry internals should name the owner module.

## Inventory

Removed root exports:

- `BoxTypeId`
- `MethodSlot`

Current root-path consumers:

- None found.

## Cleaner Boundary

```text
mir::slot_registry
  owns slot-registry type aliases and resolution helpers

mir root
  does not re-export slot-registry-local vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change slot assignment behavior.
- Do not change method slot resolution behavior.
- Do not change plugin or builtin slot fallback behavior.

## Acceptance

- MIR root no longer re-exports `BoxTypeId` or `MethodSlot`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed slot-registry-local aliases from the MIR root export surface.
- Preserved slot registry behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
