---
Status: Landed
Date: 2026-04-27
Scope: Prune unused MIR root exports for sum variant seed route vocabulary
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-496-sum-variant-tag-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-497-sum-variant-project-route-field-boundary-card.md
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-519: Sum Variant Seed Root Export Prune

## Goal

Keep sum-variant seed route and payload vocabulary owned by the seed planner
modules instead of the broad MIR root.

Function metadata already imports these route types through owner module paths.
The remaining root-level use was MIR JSON emission matching project payloads.

## Inventory

Removed root exports:

- `SumVariantTagSeedKind`
- `SumVariantTagSeedRoute`
- `SumVariantProjectSeedKind`
- `SumVariantProjectSeedPayload`
- `SumVariantProjectSeedRoute`

Current consumers:

- `src/mir/function/types.rs` imports tag/project route types through owner module paths.
- `src/runner/mir_json_emit/root.rs` now names the project payload through its owner module path.

## Cleaner Boundary

```text
sum_variant_*_seed_plan
  owns route and payload vocabulary

mir root
  exports refresh entry points only for sum variant seed lanes
```

## Boundaries

- BoxShape-only.
- Do not change tag/project route detection.
- Do not change route metadata values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports sum variant tag/project route vocabulary.
- MIR JSON emission uses the project payload owner module path.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Removed unused root-level convenience exports for sum variant seed route/payload vocabulary.
- Moved MIR JSON project payload matching to the owner module path.
- Preserved route metadata, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
