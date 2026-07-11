---
Status: Landed
Date: 2026-04-27
Scope: Prune storage and value-consumer semantic metadata root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/placement_effect.rs
  - src/mir/printer.rs
  - src/mir/string_direct_set_window_plan.rs
  - src/runner/mir_json_emit/tests/placement.rs
  - src/runner/mir_json_emit/tests/string_corridor.rs
---

# 291x-526: Storage / Value-Consumer Root Export Prune

## Goal

Keep standalone semantic metadata vocabulary owned by its modules instead of
the broad MIR root.

`StorageClass` and `ValueConsumerFacts` are internal facts. The root can keep
refresh entry points for orchestration, while fixture/test constructors import
the owner modules directly.

## Inventory

Removed root exports:

- `StorageClass`
- `ValueConsumerFacts`

Migrated consumers:

- `src/mir/placement_effect.rs`
- `src/mir/printer.rs`
- `src/mir/string_direct_set_window_plan.rs`
- `src/runner/mir_json_emit/tests/placement.rs`
- `src/runner/mir_json_emit/tests/string_corridor.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`
- `src/mir/agg_local_scalarization.rs`

## Cleaner Boundary

```text
storage_class
  owns StorageClass

value_consumer
  owns ValueConsumerFacts

mir root
  exports refresh_function_* / refresh_module_* entry points only
```

## Boundaries

- BoxShape-only.
- Do not change storage-class inference.
- Do not change value-consumer facts.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `StorageClass`.
- MIR root no longer re-exports `ValueConsumerFacts`.
- Internal consumers use `crate::mir::storage_class` or
  `crate::mir::value_consumer`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed standalone semantic fact vocabulary from the MIR root export surface.
- Kept refresh entry points available at the MIR root.
- Preserved fact collection and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
