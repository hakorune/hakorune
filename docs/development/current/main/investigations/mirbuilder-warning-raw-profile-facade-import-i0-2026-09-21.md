---
Status: fast__2026-09-21__WarningRawProfileFacadeImport__ExecuteOneImportDeletion
Task: MIRBUILDER-WARNING-RAW-PROFILE-FACADE-IMPORT-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i64-2026-09-21.md
Implementation permission: true for one bounded import deletion and one test path move only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I65
---

# Warning cleanup: Raw profile facade import

## Six-line brief

```text
Decision: remove only the unused parent-facade re-export of
  RawPublishedCompileProfileV1 and make the existing VM-reference test import
  its owning module directly.
Source authority + canonical issuer: raw_vm_reference_contract.rs owns the
  profile; the I64 two-surface warning inventory proves the parent import is
  test-only.
Non-authority: warning counts alone, cargo-fix, wildcard imports, visibility
  widening, VM policy changes, or raw-route interpretation.
Fail-fast boundary: any production reference, second test caller, compile
  error, new warning/failure name, or changed test behavior rejects the row.
Smallest next slice: delete one name from src/mir/mod.rs and change one test
  path to crate::mir::raw_vm_reference_contract::RawPublishedCompileProfileV1.
Non-claims: no Raw VM route retirement, profile redesign, suppression,
  dead-code cleanup, or semantic change.
```

## Preconditions

The current warning inventory is lib **1,755** and lib-test **560**. The only
parent-facade caller is the test assertion in
`src/runner/reference/normal_file_vm_frontdoor/tests.rs`; the profile's
definition and production uses remain in
`src/mir/raw_vm_reference_contract.rs`. The child module is already
`pub(crate)`, so the test can name the canonical owner without changing
visibility.

## Acceptance

Implementation is forbidden until the pointer leaves this design-stop card
for a fast-mode execution row. Then perform exactly the two edits named above
and run, sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo fmt --all -- --check
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Expected warning counts are lib **1,754** and lib-test **560**. Any additional
source/test reference, compile error, new warning or red name, or behavior
change rejects the deletion and restores the parent import before returning to
the next baseline refresh. No `#[allow]`, cargo-fix, or neighboring import
cleanup is allowed.
