---
Status: Landed
Date: 2026-04-27
Scope: Make ArrayStringLenWindowRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-503-array-rmw-window-route-field-boundary-card.md
  - src/mir/array_string_len_window_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/array_routes.rs
---

# 291x-504: Array String-Len Window Route Field Boundary

## Goal

Keep the `array.get(i) -> length` window route field layout, mode vocabulary,
and proof vocabulary owned by `array_string_len_window_plan`.

The route record remains public as function metadata. JSON emission should read
the stable route view, not match route-local enums or fields directly.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - reads route fields directly
  - matches `ArrayStringLenWindowMode` directly to produce JSON flags/effects
- `src/runner/mir_json_emit/tests/array_routes.rs`
  - manually constructs route records and imports route-local mode/proof
    vocabulary through the root `crate::mir` surface
- `src/mir/mod.rs`
  - root-exports the route record plus mode/proof vocabulary

Owner-local consumers:

- `src/mir/array_string_len_window_plan.rs`
  - owns detection, route field derivation, mode vocabulary, proof vocabulary,
    and focused tests

## Cleaner Boundary

```text
array_string_len_window_plan
  owns route fields, mode, proof, and JSON-facing flag/effect view
  exposes stable read accessors
  exposes cfg(test) fixture builders for JSON tests

runner JSON emitter/tests
  consume accessors/fixtures only
```

## Boundaries

- BoxShape-only.
- Do not change string-len window detection.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not add new string-len window route variants.

## Acceptance

- `ArrayStringLenWindowRoute` fields are private.
- `ArrayStringLenWindowMode` and `ArrayStringLenWindowProof` are owner-private
  and no longer root-exported.
- JSON emitter reads through route accessors only.
- Runner JSON tests use owner `cfg(test)` fixtures.
- `cargo check -q` passes.
- Array string-len window focused tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- `ArrayStringLenWindowRoute` fields are owner-private.
- `ArrayStringLenWindowMode` and `ArrayStringLenWindowProof` are
  owner-private and no longer root-exported through `crate::mir`.
- Added stable read accessors for JSON emission, including mode/proof strings,
  JSON flags, and effect tags.
- Added owner `cfg(test)` fixtures for the runner JSON test.
- Updated JSON emission and tests to consume accessors/fixtures rather than
  route-local field layout or enums.
- Preserved string-len window detection, JSON field names/values, `.inc`
  behavior, helper symbols, and lowering.

## Verification

```bash
cargo check -q
cargo test -q array_string_len
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

`tools/checks/dev_gate.sh quick` passed with the pre-existing chip8 release
artifact sync warning still reported before the final profile success line.
