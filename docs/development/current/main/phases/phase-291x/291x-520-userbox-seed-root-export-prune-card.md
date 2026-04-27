---
Status: Landed
Date: 2026-04-27
Scope: Prune unused MIR root exports for userbox seed route vocabulary
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-498-userbox-local-scalar-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-500-userbox-known-receiver-method-route-field-boundary-card.md
  - src/mir/mod.rs
  - src/mir/exact_seed_backend_route.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-520: Userbox Seed Root Export Prune

## Goal

Keep userbox seed route vocabulary owned by the seed planner modules instead of
the broad MIR root.

The remaining root-level uses were MIR JSON payload matching and exact backend
route selection. Both can consume owner module paths directly.

## Inventory

Removed root exports:

- `UserBoxLocalScalarSeedKind`
- `UserBoxLocalScalarSeedPayload`
- `UserBoxLocalScalarSeedRoute`
- `UserBoxLocalScalarSeedSinglePayload`
- `UserBoxKnownReceiverMethodSeedKind`
- `UserBoxKnownReceiverMethodSeedPayload`

Current consumers:

- `src/mir/function/types.rs` imports route types through owner module paths.
- `src/mir/exact_seed_backend_route.rs` imports local-scalar kind through the owner module path.
- `src/runner/mir_json_emit/root.rs` imports local-scalar and known-receiver vocabulary through owner module paths.

## Cleaner Boundary

```text
userbox_*_seed_plan
  owns seed route and payload vocabulary

mir root
  exports refresh entry points only for these seed lanes
```

## Boundaries

- BoxShape-only.
- Do not change route detection.
- Do not change route metadata values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports the userbox seed vocabulary listed above.
- Exact backend route selection uses owner module path for local-scalar kind.
- MIR JSON emission uses owner module paths for userbox seed vocabulary.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Removed unused root-level convenience exports for userbox local-scalar and known-receiver seed vocabulary.
- Moved exact backend route selection and MIR JSON emission to owner module imports.
- Preserved route metadata, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
