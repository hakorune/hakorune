---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextStateResidenceIndexOfSeedPayload fields owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-513-array-text-state-residence-route-contract-field-boundary-card.md
  - docs/development/current/main/phases/phase-291x/291x-501-indexof-search-micro-route-field-boundary-card.md
  - src/mir/array_text_state_residence_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-514: Array Text State Residence Payload Field Boundary

## Goal

Keep the temporary array/text state residence indexOf seed payload fields owned
by `array_text_state_residence_plan`.

The payload remains an explicit quarantined bridge until the residence emitter
becomes generic, but MIR JSON emission and focused tests should read it through
accessors instead of direct fields.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - serializes temporary indexOf seed payload fields into MIR JSON metadata
- `src/mir/mod.rs`
  - root-exports the temporary payload type even though only the owner module and JSON emitter need it

Owner-local consumer:

- `src/mir/array_text_state_residence_plan.rs`
  - owns payload construction and focused state residence tests

## Cleaner Boundary

```text
array_text_state_residence_plan
  owns temporary payload fields and payload vocabulary projection
  exposes stable payload accessors

JSON / tests
  consume payload accessors only
```

## Boundaries

- BoxShape-only.
- Do not change state residence route detection.
- Do not change contract values.
- Do not change temporary indexOf seed payload values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not retire the temporary payload in this card.

## Acceptance

- `ArrayTextStateResidenceIndexOfSeedPayload` fields are private.
- Temporary payload values are exposed through accessors.
- MIR JSON emission reads the temporary payload through accessors.
- MIR root no longer re-exports the temporary payload type.
- Focused state residence test reads the payload through accessors.
- `cargo check -q` passes.
- Focused state residence test passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made `ArrayTextStateResidenceIndexOfSeedPayload` fields owner-private.
- Added stable payload accessors for literals, lengths, proof/action/result-use, and candidate outcomes.
- Moved MIR JSON emission and the focused state residence test to payload accessors.
- Pruned the MIR root re-export for the temporary payload type.
- Preserved state residence detection, contract values, payload values, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

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
