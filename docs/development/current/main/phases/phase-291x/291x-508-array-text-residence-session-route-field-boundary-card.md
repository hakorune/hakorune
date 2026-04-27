---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayTextResidenceSessionRoute and executor contract fields owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-505-array-text-loopcarry-route-field-boundary-card.md
  - src/mir/array_text_residence_session_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
  - src/mir/passes/string_corridor_sink/tests/benchmarks.rs
---

# 291x-508: Array Text Residence Session Route Field Boundary

## Goal

Keep the array/text residence session route field layout and executor contract
vocabulary owned by `array_text_residence_session_plan`.

The session route remains public function metadata, but JSON emission and
focused tests should consume stable accessors instead of reading session and
executor contract fields directly.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - reads session route fields and executor contract fields directly
- `src/mir/passes/string_corridor_sink/tests/benchmarks.rs`
  - reads session placement/proof fields and executor contract fields directly
- `src/mir/mod.rs`
  - root-exports session scope/proof vocabulary

Owner-local consumers:

- `src/mir/array_text_residence_session_plan.rs`
  - owns session derivation, placement vocabulary, proof vocabulary, and executor contract construction

## Cleaner Boundary

```text
array_text_residence_session_plan
  owns session fields, placement/proof vocabulary, and executor contract fields
  exposes stable read accessors

JSON / focused tests
  consume accessors only
```

## Boundaries

- BoxShape-only.
- Do not change residence-session derivation.
- Do not change executor contract values.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not change loop-region mapping field layout in this card.

## Acceptance

- `ArrayTextResidenceSessionRoute` fields are private.
- `ArrayTextResidenceExecutorContract` fields are private.
- Session placement/scope/proof vocabulary is owner-private and no longer root-exported.
- JSON emission and focused benchmark checks read through session/contract accessors.
- `cargo check -q` passes.
- Focused loopcarry residence benchmark test passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- Made `ArrayTextResidenceSessionRoute` field layout owner-private.
- Made `ArrayTextResidenceExecutorContract` field layout owner-private.
- Made session placement/scope/proof vocabulary owner-private and removed root exports.
- Moved MIR JSON emission to session and executor-contract accessors.
- Moved focused residence-session benchmark checks to session and executor-contract accessors.
- Preserved residence-session derivation, executor contract values, JSON field names/values, `.inc` behavior, helper symbols, and lowering.

## Verification

```bash
cargo fmt
cargo check -q
cargo test -q benchmark_meso_substring_concat_array_set_loopcarry_has_len_store_route
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
