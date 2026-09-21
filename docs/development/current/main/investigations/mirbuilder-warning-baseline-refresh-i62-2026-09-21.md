---
Status: closed__2026-09-21__WarningBaselineRefreshI62__SelectedSemanticPackageTestReexports
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I62
Date: 2026-09-21
Parent: mirbuilder-warning-vm-reference-test-import-i0-2026-09-21.md
Implementation permission: true for the selected two test-only semantic-package re-exports only
NextCard: MIRBUILDER-WARNING-SEMANTIC-PACKAGE-TEST-REEXPORTS-I0
---

# MirBuilder warning baseline refresh I62

## Six-line brief

```text
Decision: refresh both warning surfaces after the VM-reference test-import
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,759/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
 their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,759** and lib-test **561** after the
selected VM-reference test-import cohort.


## Selection evidence

The fixed gates completed sequentially with exit 0 after the VM-reference cohort:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,759 | `/tmp/hakorune-warning-i62-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i62-lib-test-20260921.log` |

The first two warning groups are the unused `NormalCallableSemanticPackageIssueV1`
and `NormalCallableDynamicProjectionRefV1` re-exports at
`src/mir/normal_callable_semantic_package/mod.rs:103,113`. A source census shows
both parent re-exports are consumed only by `#[cfg(test)]` fixtures; production
issuer and model code use their owning submodules directly. The bounded same-owner
slice gates these two test-only re-exports together, with expected lib **1,759 →
1,757** and lib-test unchanged at **561**. No semantic-package issuer, model, or
test body is selected.
