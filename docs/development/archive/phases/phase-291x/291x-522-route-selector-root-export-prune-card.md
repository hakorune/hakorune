---
Status: Landed
Date: 2026-04-27
Scope: Prune unused MIR root exports for route selector/helper vocabulary
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/mod.rs
  - src/mir/exact_seed_backend_route.rs
  - src/mir/userbox_loop_micro_seed_plan.rs
---

# 291x-522: Route Selector Root Export Prune

## Goal

Keep route selector/helper vocabulary owned by its planner module instead of the
broad MIR root.

After the route root-export cleanup, `ExactSeedBackendRoute` and
`UserBoxLoopMicroSeedKind` no longer need root-level convenience exports.

## Inventory

Removed root exports:

- `ExactSeedBackendRoute`
- `UserBoxLoopMicroSeedKind`

Current consumers:

- Function metadata imports `ExactSeedBackendRoute` through
  `exact_seed_backend_route`.
- Userbox loop micro route construction and tests use `UserBoxLoopMicroSeedKind`
  inside `userbox_loop_micro_seed_plan`.

## Cleaner Boundary

```text
exact_seed_backend_route / userbox_loop_micro_seed_plan
  own selector/helper vocabulary

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

- MIR root no longer re-exports `ExactSeedBackendRoute`.
- MIR root no longer re-exports `UserBoxLoopMicroSeedKind`.
- Owner-module consumers continue to compile.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Removed unused root-level convenience exports for exact backend route and userbox loop micro kind.
- Preserved route metadata, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
