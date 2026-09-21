---
Status: closed__2026-09-21__WarningBaselineRefreshI54__ExternalCommitTestImports
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I54
Date: 2026-09-21
Parent: mirbuilder-warning-direct-accum-profile-test-imports-i0-2026-09-21.md
Implementation permission: selection recorded; execution completed by MIRBUILDER-WARNING-EXTERNAL-COMMIT-TEST-IMPORTS-I0
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I55
---

# MirBuilder warning baseline refresh I54

## Six-line brief

```text
Decision: refresh both warning surfaces after the I53 DirectAccum test-import
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,769/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each, sequentially. Record
lint, file:line, owner, and production/test/compat/generated role, then select
one cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

The previous fixed baseline is lib **1,769** and lib-test **561** after I53.


## Selection evidence

The fixed gates completed sequentially at the current head:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,769 | `/tmp/hakorune-warning-i54-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i54-lib-test-20260921.log` |

The selected caller-zero cohort is the two final-verification seal imports in
`src/mir/compiler/external_commit.rs:7`:

* `CanonicalFinalVerificationSealInnerV1`
* `CanonicalFinalVerificationSealV1`

The finite source census found both names only in the module's
`#[cfg(test)] mod tests` fixture. Production external commit uses
`ModuleVerificationEvidenceV1` and constructs the seal through the canonical
postprocess path; no production caller reaches these imports.

The bounded execution slice gates only these two imports under `#[cfg(test)]`;
external commit preparation, publication, and verification behavior remain
unchanged.

## Closeout evidence

The two selected final-verification seal imports were gated under `#[cfg(test)]`.
Both fixed gates exited 0:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,768 | `/tmp/hakorune-warning-i0-external-commit-test-imports-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i0-external-commit-test-imports-lib-test-20260921.log` |

I54 closes with no external commit preparation, publication, or verification
behavior change.
