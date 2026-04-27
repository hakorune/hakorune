---
Status: Landed
Date: 2026-04-27
Scope: Prune unused MIR root exports for remaining seed/window route types
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/array_rmw_add1_leaf_seed_plan.rs
---

# 291x-521: Seed Window Route Root Export Prune

## Goal

Keep remaining seed/window route type names owned by their planner modules
instead of the broad MIR root.

Function metadata already imports these route types through owner module paths.
The only root-level use left in this slice was a test-support import of
`ArrayRmwWindowProof`.

## Inventory

Removed root exports:

- `ArrayRmwAdd1LeafSeedRoute`
- `ArrayRmwWindowProof`
- `ArrayRmwWindowRoute`
- `ArrayStringLenWindowRoute`
- `ArrayStringStoreMicroSeedRoute`
- `ConcatConstSuffixMicroSeedRoute`
- `StringDirectSetWindowRoute`
- `SubstringViewsMicroSeedRoute`

Current consumers:

- `src/mir/function/types.rs` imports route types through owner module paths.
- `src/mir/array_rmw_add1_leaf_seed_plan.rs` test support now imports
  `ArrayRmwWindowProof` through the owner module path.

## Cleaner Boundary

```text
*_seed_plan / *_window_plan
  owns route and proof type names

mir root
  exports refresh entry points only for these lanes
```

## Boundaries

- BoxShape-only.
- Do not change route detection.
- Do not change route metadata values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports the seed/window route/proof types listed above.
- Owner-module imports continue to compile.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Removed unused root-level convenience exports for remaining seed/window route types.
- Moved the lone `ArrayRmwWindowProof` test-support import to the owner module path.
- Preserved route metadata, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
