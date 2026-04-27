---
Status: Landed
Date: 2026-04-27
Scope: Keep ArrayTextCombinedRegion effect/consumer vocabulary owner-local
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-507-array-text-combined-region-route-field-boundary-card.md
  - src/mir/array_text_combined_region_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
  - src/mir/passes/string_corridor_sink/tests/benchmarks.rs
---

# 291x-512: Array Text Combined Region Vocabulary Boundary

## Goal

Keep array/text combined-region effect and consumer-capability vocabulary owned by
`array_text_combined_region_plan`.

Combined-region metadata consumers should read route accessors for carrier,
effects, consumer capabilities, publication boundary, and materialization policy
instead of duplicating the same strings in JSON emitters or importing route
vocabulary enums from the MIR root.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - hard-codes combined-region effects, consumer capabilities, carrier, publication boundary, and materialization policy
- `src/mir/mod.rs`
  - root-exports combined-region effect/consumer vocabulary types
- `src/mir/passes/string_corridor_sink/tests/benchmarks.rs`
  - focused combined-region test verifies the public route metadata contract

Owner-local consumer:

- `src/mir/array_text_combined_region_plan.rs`
  - owns combined-region route construction and semantic metadata vocabulary

## Cleaner Boundary

```text
array_text_combined_region_plan
  owns route fields and semantic metadata vocabulary
  exposes stable route accessors

JSON / tests
  consume route accessors only
```

## Boundaries

- BoxShape-only.
- Do not change combined-region route detection.
- Do not change combined-region metadata values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not change residence or observer executor contracts in this card.

## Acceptance

- Combined-region effect and consumer-capability enums are owner-private.
- MIR root no longer re-exports combined-region effect/consumer vocabulary.
- Combined-region route exposes accessors for carrier/effects/consumer capabilities/publication boundary/materialization policy.
- MIR JSON emission reads combined-region semantic metadata through route accessors.
- Focused combined-region test verifies the route accessors.
- `cargo check -q` passes.
- Focused combined-region test passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made `ArrayTextCombinedRegionEffect` and `ArrayTextCombinedRegionConsumerCapability` owner-private.
- Added route accessors for publication boundary, carrier, effects, consumer capabilities, and materialization policy.
- Moved MIR JSON emission from duplicated combined-region metadata strings to route accessors.
- Pruned MIR root exports for combined-region effect/consumer vocabulary.
- Extended the focused combined-region test to verify the semantic metadata accessors.
- Preserved combined-region route detection, metadata values, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q benchmark_kilo_kernel_small_has_combined_edit_observer_region
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
