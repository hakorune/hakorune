---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextObserverExecutorContract and ArrayTextObserverStoreRegionMapping fields owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-509-array-text-observer-route-field-boundary-card.md
  - src/mir/array_text_observer_region_contract.rs
  - src/mir/array_text_combined_region_plan.rs
  - src/mir/array_text_observer_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-510: Array Text Observer Executor Contract Field Boundary

## Goal

Keep the nested array/text observer executor contract and store-region mapping
owned by `array_text_observer_region_contract`.

Observer routes may expose an executor contract, but combined-region derivation,
MIR JSON emission, root exports, and focused tests should consume accessors
instead of reading the contract or mapping fields directly.

## Inventory

Current external consumers:

- `src/mir/array_text_combined_region_plan.rs`
  - checks executor mode and consumes observer store-region mapping while deriving combined regions
- `src/runner/mir_json_emit/root.rs`
  - serializes observer executor contract and region mapping into MIR JSON metadata
- `src/mir/array_text_observer_plan.rs`
  - focused tests inspect the nested executor contract
- `src/mir/mod.rs`
  - root-exports observer executor vocabulary that should remain owner-local

Owner-local consumer:

- `src/mir/array_text_observer_region_contract.rs`
  - owns executor vocabulary, contract construction, and store-region mapping derivation

## Cleaner Boundary

```text
array_text_observer_region_contract
  owns executor vocabulary, contract fields, and store-region mapping fields
  exposes stable read accessors and owner predicates

combined-region / JSON / tests
  consume accessors and predicates only
```

## Boundaries

- BoxShape-only.
- Do not change observer route detection.
- Do not change observer executor contract values.
- Do not change observer store-region mapping values.
- Do not change combined-region derivation semantics.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not change array/text residence-session contract or mapping fields in this card.

## Acceptance

- `ArrayTextObserverExecutorContract` fields are private.
- `ArrayTextObserverStoreRegionMapping` fields are private.
- Observer executor vocabulary remains owner-local and is no longer root-exported.
- Combined-region derivation reads observer executor contract and mapping through accessors/predicates.
- MIR JSON emission reads observer executor contract and mapping through accessors.
- Focused observer tests read observer executor contract and mapping through accessors.
- `cargo check -q` passes.
- Focused observer and combined-region tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made observer executor vocabulary owner-private inside `array_text_observer_region_contract`.
- Made `ArrayTextObserverExecutorContract` field layout owner-private.
- Made `ArrayTextObserverStoreRegionMapping` field layout owner-private.
- Added stable read accessors for contract tags, consumer/effect vocabularies, and region mapping.
- Added an owner predicate for the single-region executor mode.
- Moved combined-region derivation, MIR JSON emission, and focused observer tests to accessors/predicates.
- Pruned root exports for observer executor vocabulary that callers no longer need directly.
- Preserved observer route detection, executor contract values, mapping values, combined-region semantics, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q attaches_executor_contract_for_observer_conditional_suffix_store_region
cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
