---
Status: Landed
Date: 2026-04-27
Scope: Prune unused MIR root export for ArrayTextStateResidenceRoute
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-514-array-text-state-residence-payload-field-boundary-card.md
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-516: Array Text State Residence Root Export Prune

## Goal

Keep `ArrayTextStateResidenceRoute` reachable through
`array_text_state_residence_plan` instead of the broad MIR root.

The route remains public because function metadata carries it, but the root-level
convenience export was only used by MIR JSON emission.

## Inventory

Current root export:

- `ArrayTextStateResidenceRoute`

Current consumers:

- `src/mir/function/types.rs` already imports the route through the owner module.
- `src/runner/mir_json_emit/root.rs` used the root export for a helper signature.

## Cleaner Boundary

```text
array_text_state_residence_plan
  owns the route type name

mir root
  exports refresh entry points only for this state-residence lane
```

## Boundaries

- BoxShape-only.
- Do not change state residence route detection.
- Do not change contract or temporary payload values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- MIR root no longer re-exports `ArrayTextStateResidenceRoute`.
- MIR JSON emission names the route through the owner module path.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Removed the unused root-level convenience export for `ArrayTextStateResidenceRoute`.
- Moved the MIR JSON helper signature to the owner module path.
- Preserved state residence metadata values, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
