---
Status: Landed
Date: 2026-04-27
Scope: Prune CoreMethod and GenericMethod route vocabulary root exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/generic_method_route_plan.rs
  - src/mir/map_lookup_fusion_plan.rs
---

# 291x-530: Core / Generic Method Root Export Prune

## Goal

Keep CoreMethod and GenericMethod route vocabulary owned by their modules
instead of the broad MIR root.

The MIR root should expose refresh entry points for orchestration. Planner
internals that inspect CoreMethod/GenericMethod facts should import owner
modules directly.

## Inventory

Removed root exports:

- `CoreMethodLoweringTier`
- `CoreMethodOp`
- `CoreMethodOpCarrier`
- `CoreMethodOpProof`
- `GenericMethodKeyRoute`
- `GenericMethodPublicationPolicy`
- `GenericMethodReturnShape`
- `GenericMethodValueDemand`
- `GenericMethodRoute`

Migrated consumers:

- `src/mir/generic_method_route_plan.rs`
- `src/mir/map_lookup_fusion_plan.rs`

Existing owner imports already covered:

- `src/mir/function/types.rs`

## Cleaner Boundary

```text
core_method_op
  owns CoreMethodOp* vocabulary

generic_method_route_facts / generic_method_route_plan
  own GenericMethod* vocabulary

mir root
  exports refresh_function_generic_method_routes
  exports refresh_module_generic_method_routes
```

## Boundaries

- BoxShape-only.
- Do not change route detection.
- Do not change CoreMethod metadata.
- Do not change MapGet/MapHas fusion preflight behavior.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports CoreMethod vocabulary.
- MIR root no longer re-exports GenericMethod vocabulary.
- Planner consumers use owner modules.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed CoreMethod/GenericMethod contract vocabulary from the MIR root export
  surface.
- Kept GenericMethod refresh entry points available at the MIR root.
- Preserved planner behavior and route metadata.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
