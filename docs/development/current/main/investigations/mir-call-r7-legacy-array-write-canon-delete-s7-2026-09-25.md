# MIR-CALL-R7-LEGACY-ARRAY-WRITE-CANON-DELETE-S7 — delete dead array-write repair

Status: landed__2026-09-25
Date: 2026-09-25
Parent: MIR-CALL-R7-CALLER-ZERO-D2 (accepted, bounded)
Owner card:
  docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md
Authority: docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
             Call/R7 frontier.

## Slice

Delete the legacy-carrier array-write repair path:

- `src/mir/array_element_write.rs`:
  `canonicalize_legacy_array_write_calls` (:105-170) — the whole
  function. It only matches `LegacyCallV0{Method{ArrayBox,
  push/set/insert, receiver:Some}}` and rewrites it to
  `ArrayElementWrite`.
- `src/mir/semantic_refresh/contracts.rs`: remove the call at
  :116-117 (`canonicalize_legacy_array_write_calls(function)` +
  `map_err` wrapper).

## Fail-fast boundary

Strengthening deletion: a residual
`LegacyCallV0{ArrayBox.push/set/insert}` row is no longer silently
upgraded — `refresh_function_array_write_witnesses` (called
immediately after in `refresh_active_contract_carriers`) runs
`rebuild -> reject_residual_calls`, which rejects the row with
`[mir/array_write/residual_call]`.

Pin: a synthetic function carrying a legacy ArrayBox `push` call row
must produce the residual named-stop from
`refresh_function_array_write_witnesses` (or
`refresh_module_semantic_metadata`) — not an upgrade.

## Verification (landed)

- `cargo check --profile quick` clean.
- `cargo test --profile quick --lib array_element_write`: 8/9 green;
  the one red (`published_array_element_writes_compile_object_..`) is
  baseline debt (`cargo_lib_red_baseline.*`, llvm-mem2reg toolchain
  failure), not this change.
- New pin green:
  `residual_legacy_array_push_call_rejects_instead_of_upgrading` —
  a residual `LegacyCallV0{ArrayBox.push}` now hits
  `[mir/array_write/residual_call]` instead of being upgraded.
- `semantic_refresh` tests 10/10 green.
- `mir_call_d1b_active_surface_guard` row
  `MIR-CALL-R7-LEGACY-ARRAY-WRITE-CANON-DELETE-S7` registered in r6
  helpers and green; pointer guard green; lifecycle inventory
  re-pinned; `git diff --check` clean.

## Exit

- [x] Deletion set exactly matches this card (function + call site
      only; `reject_residual_calls`,
      `project_module_to_legacy_calls`, and the
      `LegacyCanonicalized` producer-kind tag retained — its wire tag
      is shared by test fixtures and stays for its own slice).
- [x] Pins + focused suites green; new reds classified (baseline only).
- [x] Guard row registered and green; card/pointer/workstream synced.
