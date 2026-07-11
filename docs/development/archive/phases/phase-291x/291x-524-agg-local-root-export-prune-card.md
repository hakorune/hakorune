---
Status: Landed
Date: 2026-04-27
Scope: Prune agg-local semantic metadata root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/placement_effect.rs
  - src/runner/mir_json_emit/agg_local.rs
  - src/runner/mir_json_emit/tests/placement.rs
---

# 291x-524: Agg-Local Root Export Prune

## Goal

Keep agg-local semantic metadata owned by `agg_local_scalarization` instead of
the broad MIR root.

`AggLocalScalarizationKind` and `AggLocalScalarizationRoute` are internal
semantic metadata vocabulary. Refresh entry points can stay easy to call from
the MIR root, but consumers that construct or inspect the vocabulary should
import the owner module directly.

## Inventory

Removed root exports:

- `AggLocalScalarizationKind`
- `AggLocalScalarizationRoute`

Migrated consumers:

- `src/mir/placement_effect.rs`
- `src/runner/mir_json_emit/agg_local.rs`
- `src/runner/mir_json_emit/tests/placement.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`

## Cleaner Boundary

```text
agg_local_scalarization
  owns AggLocalScalarizationKind / AggLocalScalarizationRoute

mir root
  exports refresh_function_agg_local_scalarization_routes
  exports refresh_module_agg_local_scalarization_routes
```

## Boundaries

- BoxShape-only.
- Do not change route collection.
- Do not change placement/effect route derivation.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `AggLocalScalarizationKind`.
- MIR root no longer re-exports `AggLocalScalarizationRoute`.
- Internal consumers use `crate::mir::agg_local_scalarization`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed agg-local semantic vocabulary from the MIR root export surface.
- Kept agg-local refresh entry points available at the MIR root.
- Preserved metadata values and JSON output.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
