---
Status: Landed
Date: 2026-04-27
Scope: Prune route root exports used only by MIR JSON helper signatures
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-494-array-getset-micro-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-499-userbox-loop-micro-route-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-500-userbox-known-receiver-method-route-field-boundary-card.md
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-518: JSON Helper Route Root Export Prune

## Goal

Keep route type names owned by their planner modules when the only root-export
consumer is a MIR JSON helper signature.

The route types remain public through their owner modules because function
metadata carries them. MIR JSON emission can name those owner module paths
directly.

## Inventory

Removed root exports:

- `ArrayGetSetMicroSeedRoute`
- `UserBoxLoopMicroSeedRoute`
- `UserBoxKnownReceiverMethodSeedRoute`

Current root-export consumers:

- `src/runner/mir_json_emit/root.rs` helper signatures only

## Cleaner Boundary

```text
*_seed_plan
  owns route type names

mir_json_emit
  consumes owner module paths

mir root
  does not re-export helper-only route names
```

## Boundaries

- BoxShape-only.
- Do not change route detection.
- Do not change route metadata values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports the three route types listed above.
- MIR JSON helper signatures use owner module paths.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Removed helper-only route root exports for array get/set, userbox loop, and userbox known-receiver routes.
- Moved MIR JSON helper signatures to owner module paths.
- Preserved route metadata, JSON, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
