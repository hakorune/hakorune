---
Status: closed__2026-09-21__WarningBaselineRefreshI64__SelectedRawProfileFacadeImport
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I64
Date: 2026-09-21
Parent: mirbuilder-warning-birth-result-reexport-i0-2026-09-21.md
Implementation permission: false; selection recorded, successor must enter fast mode before code
NextCard: MIRBUILDER-WARNING-RAW-PROFILE-FACADE-IMPORT-I0
---

# MirBuilder warning baseline refresh I64

## Six-line brief

```text
Decision: refresh both warning surfaces after the caller-zero birth-result
  re-export cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,756/560 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
 their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,756** and lib-test **560** after the
selected caller-zero birth-result re-export cohort.

## Selection evidence

The two fixed gates completed sequentially with exit 0 at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,755 | `/tmp/hakorune-warning-i64-lib-20260921.log` |
| lib test | 560 | `/tmp/hakorune-warning-i64-lib-test-20260921.log` |

The lib count is one below the fixed 1,756 baseline because the recent
composite loop projection now consumes `VerifiedLoopCondSourceExitV1::record`;
that is a current-change warning disappearance, not a warning-cleanup
deletion. The lib-test count is unchanged. Line shifts in the loop source
accessor warnings are likewise inventory noise and do not authorize a broad
cleanup.

The selected bounded cohort is the unused `RawPublishedCompileProfileV1`
re-export at `src/mir/mod.rs:123`. A complete source census finds the type's
definition and internal raw-profile uses in
`raw_vm_reference_contract.rs`, plus one test-only use through
`crate::mir::RawPublishedCompileProfileV1` in
`runner/reference/normal_file_vm_frontdoor/tests.rs`; no production caller
uses the parent facade. The successor may remove only that parent import and
change the test to the owning `crate::mir::raw_vm_reference_contract` path.
Expected result is lib **1,755 → 1,754**, lib-test unchanged at **560**.
No raw VM route, profile owner, visibility, or test behavior is selected.

## Closeout boundary

This refresh is documentation-only. The selected deletion is not performed
while `work_mode = "design_stop"`; the successor must first record the fast
mode transition, then run the two fixed checks, `cargo fmt --all -- --check`,
`git diff --check`, and the current-state pointer guard. Any additional
reference, compile failure, new warning family, or changed red name rejects
the deletion and returns to the warning census. No suppression, cargo-fix,
dead-code cleanup, or semantic route change is authorized.
