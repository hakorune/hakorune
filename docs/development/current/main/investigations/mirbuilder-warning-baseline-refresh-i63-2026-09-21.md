---
Status: closed__2026-09-21__WarningBaselineRefreshI63__SelectedBirthResultReexport
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I63
Date: 2026-09-21
Parent: mirbuilder-warning-semantic-package-test-reexports-i0-2026-09-21.md
Implementation permission: true for the selected caller-zero BirthResultAbiV1 re-export only
NextCard: MIRBUILDER-WARNING-BIRTH-RESULT-REEXPORT-I0
---

# MirBuilder warning baseline refresh I63

## Six-line brief

```text
Decision: refresh both warning surfaces after the semantic-package test
  re-export cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,757/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
 their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,757** and lib-test **561** after the
selected semantic-package test re-export cohort.


## Selection evidence

The fixed gates completed sequentially with exit 0 after the semantic-package
re-export cohort:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,757 | `/tmp/hakorune-warning-i63-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i63-lib-test-20260921.log` |

The first warning group is the unused `BirthResultAbiV1` re-export at
`src/mir/normal_callable_semantic_package/ordinary_new_coseal.rs:17`. A full source census shows
no caller of the ordinary_new_coseal re-export; the type is used only inside its owning
`birth_abi_handoff` module. The bounded slice is therefore caller-zero deletion
of one internal re-export, with expected lib **1,757 → 1,756** and lib-test
unchanged at **561**. No birth ABI owner or test body is selected.


## Execution correction

The selected caller-zero re-export was found at the inner
`ordinary_new_coseal.rs:17` owner rather than the parent facade. The corrected
execution removed one warning from both surfaces: lib **1,757 → 1,756** and
lib-test **561 → 560**. The parent-facade edit remains part of the same bounded
caller-zero cleanup and has no production caller.
