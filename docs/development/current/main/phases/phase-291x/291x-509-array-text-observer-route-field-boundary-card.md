---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextObserverRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-508-array-text-residence-session-route-field-boundary-card.md
  - src/mir/array_text_observer_plan.rs
  - src/mir/array_text_observer_region_contract.rs
  - src/mir/array_text_combined_region_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-509: Array Text Observer Route Field Boundary

## Goal

Keep the array/text observer route field layout owned by
`array_text_observer_plan`.

The observer route remains public function metadata, but observer-region
contract derivation, combined-region derivation, JSON emission, and focused
tests should consume stable accessors instead of reading route fields directly.

## Inventory

Current external consumers:

- `src/mir/array_text_observer_region_contract.rs`
  - derives nested observer-store contract from observer route fields directly
- `src/mir/array_text_combined_region_plan.rs`
  - consumes observer route array/arg/contract fields directly while deriving combined regions
- `src/runner/mir_json_emit/root.rs`
  - reads observer route fields directly for MIR JSON metadata
- `src/mir/mod.rs`
  - root-exports observer route vocabulary even though callers only need route metadata

Owner-local consumers:

- `src/mir/array_text_observer_plan.rs`
  - owns observer route detection, consumer-shape vocabulary, argument representation, and liveness flags

## Cleaner Boundary

```text
array_text_observer_plan
  owns route fields and observer vocabulary
  exposes stable read accessors and owner predicates

observer-region / combined-region / JSON / tests
  consume accessors and predicates only
```

## Boundaries

- BoxShape-only.
- Do not change observer route detection.
- Do not change observer-region contract derivation semantics.
- Do not change combined-region derivation semantics.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not change observer executor contract or store-region mapping field layout in this card.

## Acceptance

- `ArrayTextObserverRoute` fields are private.
- Observer-region and combined-region derivation read observer route metadata through accessors/predicates.
- JSON emission and focused observer tests read through route accessors.
- Root export no longer re-exports observer route vocabulary that is not needed outside the owner module.
- `cargo check -q` passes.
- Focused observer and combined-region tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made `ArrayTextObserverRoute` field layout owner-private.
- Added stable read accessors for observer site/value metadata, arg representation, liveness flags, result/consumer/proof tags, and nested executor contract.
- Moved observer-region contract derivation to observer-route owner predicates/accessors.
- Moved combined-region derivation to observer-route accessors.
- Moved MIR JSON emission and focused observer tests to observer-route accessors.
- Pruned root re-exports for observer route vocabulary that callers no longer need directly.
- Preserved observer route detection, observer-region contract derivation semantics, combined-region semantics, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q detects_array_get_indexof_found_predicate_route
cargo test -q attaches_executor_contract_for_observer_conditional_suffix_store_region
cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
