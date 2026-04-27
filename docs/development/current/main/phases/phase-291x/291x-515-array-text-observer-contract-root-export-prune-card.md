---
Status: Landed
Date: 2026-04-27
Scope: Prune unused MIR root exports for observer executor contract and mapping
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-510-array-text-observer-executor-contract-field-boundary-card.md
  - src/mir/mod.rs
  - src/mir/array_text_observer_region_contract.rs
---

# 291x-515: Array Text Observer Contract Root Export Prune

## Goal

Keep observer executor contract and store-region mapping types reachable through
their owner module instead of the broad MIR root.

The types remain public because observer route accessors expose borrowed
contract/mapping references, but no caller needs the root-level convenience
exports.

## Inventory

Current root exports:

- `ArrayTextObserverExecutorContract`
- `ArrayTextObserverStoreRegionMapping`

Current consumers:

- Owner and sibling MIR route modules import the types through
  `array_text_observer_region_contract`.
- No code consumes the types through `crate::mir::...`.

## Cleaner Boundary

```text
array_text_observer_region_contract
  owns observer executor contract and mapping type names

mir root
  exports route metadata entry points only
```

## Boundaries

- BoxShape-only.
- Do not change observer route detection.
- Do not change observer executor contract or mapping values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `ArrayTextObserverExecutorContract`.
- MIR root no longer re-exports `ArrayTextObserverStoreRegionMapping`.
- Existing owner-module imports continue to compile.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Removed unused root-level convenience exports for observer executor contract and store-region mapping.
- Preserved the owner module API and all runtime/compiler behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
