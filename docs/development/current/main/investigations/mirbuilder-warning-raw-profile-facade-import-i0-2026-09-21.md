---
Status: closed__2026-09-21__WarningRawProfileFacadeImport__OneImportDeletion
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

## Execution evidence

The bounded deletion completed with exactly the two authorized edits: the
parent `mir` facade no longer re-exports `RawPublishedCompileProfileV1`, and
the existing VM-reference test names the owning
`raw_vm_reference_contract` module directly. A post-edit source census finds
no production facade caller and no remaining parent-path caller.

The fixed gates ran sequentially and exited 0:

| gate | result | evidence |
| --- | --- | --- |
| `cargo check --profile quick --lib -j4` | lib **1,754** warnings | `/tmp/hakorune-warning-raw-profile-lib-20260921.log` |
| `cargo test --profile quick --lib --no-run -j4` | lib-test **560** warnings | `/tmp/hakorune-warning-raw-profile-lib-test-20260921.log` |
| `cargo fmt --all -- --check` | PASS | local command |
| `git diff --check` | PASS | local command |
| current-state pointer guard | PASS | local command |

No production route, Raw profile behavior, test assertion, failure name, or
warning family changed. The next action is a fresh two-surface baseline
refresh; no neighboring import, suppression, or dead-code cleanup is included.
