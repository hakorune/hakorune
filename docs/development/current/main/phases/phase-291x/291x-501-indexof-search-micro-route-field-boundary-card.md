---
Status: Landed
Date: 2026-04-27
Scope: Make IndexOfSearchMicroSeedRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-500-userbox-known-receiver-method-route-field-boundary-card.md
  - src/mir/indexof_search_micro_seed_plan.rs
  - src/mir/array_text_state_residence_plan.rs
---

# 291x-501: IndexOf Search Micro Route Field Boundary

## Goal

Keep the temporary indexOf search exact payload route record owned by
`indexof_search_micro_seed_plan`.

This route is not the active backend metadata owner. The active contract remains
`array_text_state_residence_route`, so this card only closes the temporary route
field layout. The proof enum remains public for now because the residence
payload still uses it as explicit vocabulary.

## Inventory

Current external consumers:

- `src/mir/array_text_state_residence_plan.rs`
  - imports the route type and proof/payload vocabulary
  - copies route fields directly into the active residence payload
  - test code manually constructs the exact route

Owner-local consumers:

- `src/mir/indexof_search_micro_seed_plan.rs`
  - owns detection, exact route field derivation, proof vocabulary, and tests

## Cleaner Boundary

```text
indexof_search_micro_seed_plan
  owns temporary route fields
  exposes stable read accessors
  exposes cfg(test) fixture builder for cross-module tests

array_text_state_residence_plan
  consumes accessors only
  remains active metadata owner
```

## Boundaries

- BoxShape-only.
- Do not change route detection.
- Do not change residence metadata fields or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not make `IndexOfSearchMicroSeedProof` private in this card.

## Acceptance

- `IndexOfSearchMicroSeedRoute` fields are private.
- `array_text_state_residence_plan` reads the temporary route through accessors.
- Cross-module test construction uses an owner `cfg(test)` fixture.
- `cargo check -q` passes.
- indexOf search and residence focused tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- `IndexOfSearchMicroSeedRoute` fields are owner-private.
- `IndexOfSearchMicroSeedProof` intentionally remains public because
  `ArrayTextStateResidenceIndexOfSeedPayload` still uses it as explicit
  temporary vocabulary.
- `array_text_state_residence_plan` consumes the temporary route through
  accessors only.
- Cross-module residence tests use an owner `cfg(test)` fixture instead of
  constructing the route fields directly.
- No `.inc`, helper symbol, lowering, or route detection behavior changed.

## Verification

```bash
cargo check -q
cargo test -q indexof_search_micro_seed
cargo test -q array_text_state_residence
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

`tools/checks/dev_gate.sh quick` passed with the pre-existing chip8 release
artifact sync warning still reported before the final profile success line.
