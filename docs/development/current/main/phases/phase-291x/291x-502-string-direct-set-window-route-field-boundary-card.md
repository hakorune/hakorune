---
Status: Landed
Date: 2026-04-27
Scope: Make StringDirectSetWindowRoute field layout owner-private
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-501-indexof-search-micro-route-field-boundary-card.md
  - src/mir/string_direct_set_window_plan.rs
  - src/mir/mod.rs
  - src/runner/mir_json_emit/root.rs
  - src/runner/mir_json_emit/tests/string_direct_set_routes.rs
---

# 291x-502: String Direct-Set Window Route Field Boundary

## Goal

Keep the direct-set source-window route field layout and proof vocabulary owned
by `string_direct_set_window_plan`.

The route record remains public as function metadata, but consumers should not
depend on its internal field layout. JSON emission is the only external
consumer in this card.

## Inventory

Current external consumers:

- `src/runner/mir_json_emit/root.rs`
  - reads every route field directly for JSON metadata
- `src/runner/mir_json_emit/tests/string_direct_set_routes.rs`
  - manually constructs the route and imports the proof vocabulary through the
    root `crate::mir` surface
- `src/mir/mod.rs`
  - root-exports both the route record and its proof enum

Owner-local consumers:

- `src/mir/string_direct_set_window_plan.rs`
  - owns detection, route field derivation, proof vocabulary, and focused tests

## Cleaner Boundary

```text
string_direct_set_window_plan
  owns route fields and proof vocabulary
  exposes stable read accessors for JSON
  exposes cfg(test) fixture builder for JSON tests

runner JSON emitter/tests
  consume accessors/fixtures only
```

## Boundaries

- BoxShape-only.
- Do not change route detection.
- Do not change JSON field names or values.
- Do not change `.inc` behavior, helper symbols, or lowering.
- Do not add new direct-set window route variants.

## Acceptance

- `StringDirectSetWindowRoute` fields are private.
- `StringDirectSetWindowProof` is owner-private and no longer root-exported.
- JSON emitter reads through route accessors only.
- Runner JSON direct-set route tests use owner `cfg(test)` fixtures.
- `cargo check -q` passes.
- Direct-set route focused tests pass.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.

## Result

- `StringDirectSetWindowRoute` fields are owner-private.
- `StringDirectSetWindowProof` is owner-private and no longer root-exported
  through `crate::mir`.
- Added stable read accessors for JSON emission.
- Added an owner `cfg(test)` fixture for the runner JSON test.
- Updated the JSON emitter and runner JSON test to consume accessors/fixtures
  rather than route field layout.
- Preserved route detection, JSON field names/values, `.inc` behavior, helper
  symbols, and lowering.

## Verification

```bash
cargo check -q
cargo test -q string_direct_set
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

`tools/checks/dev_gate.sh quick` passed with the pre-existing chip8 release
artifact sync warning still reported before the final profile success line.
