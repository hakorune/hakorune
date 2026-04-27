---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextStateResidenceRoute and contract fields owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-501-indexof-search-micro-route-field-boundary-card.md
  - src/mir/array_text_state_residence_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-513: Array Text State Residence Route Contract Field Boundary

## Goal

Keep the array/text state residence route and contract fields owned by
`array_text_state_residence_plan`.

The route remains the active metadata owner for the current indexOf residence
front, but MIR JSON emission and tests should read contract values through
accessors instead of directly reading route or contract fields.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - reads state residence route and contract fields directly
- `src/mir/mod.rs`
  - root-exports state residence contract/vocabulary types that should remain owner-local

Owner-local consumer:

- `src/mir/array_text_state_residence_plan.rs`
  - owns state residence contract vocabulary and route construction

Temporary child payload:

- `ArrayTextStateResidenceIndexOfSeedPayload` remains an explicit public child
  payload for the current exact indexOf bridge. Its field boundary is intentionally
  left for a separate card.

## Cleaner Boundary

```text
array_text_state_residence_plan
  owns route fields, contract fields, and contract vocabulary
  exposes stable route/contract accessors

JSON / tests
  consume route/contract accessors only
```

## Boundaries

- BoxShape-only.
- Do not change state residence route detection.
- Do not change contract values.
- Do not change temporary indexOf seed payload values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not close `ArrayTextStateResidenceIndexOfSeedPayload` fields in this card.

## Acceptance

- State residence contract vocabulary remains owner-local.
- `ArrayTextStateResidenceContract` fields are private.
- `ArrayTextStateResidenceRoute` fields are private.
- MIR root no longer re-exports state residence contract/vocabulary types.
- MIR JSON emission reads state residence route/contract through accessors.
- Focused state residence test reads route/contract through accessors.
- `cargo check -q` passes.
- Focused state residence test passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made state residence contract vocabulary owner-private.
- Made `ArrayTextStateResidenceContract` and `ArrayTextStateResidenceRoute` fields owner-private.
- Added route accessors for the contract and temporary indexOf seed payload.
- Added contract accessors for observer kind, residence, result representation, consumer capability, and publication boundary.
- Moved MIR JSON emission and the focused state residence test to accessors.
- Pruned MIR root exports for state residence contract/vocabulary types.
- Preserved state residence detection, contract values, temporary payload values, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q derives_residence_route_from_indexof_search_route
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
