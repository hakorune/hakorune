---
Status: design_stop__2026-09-21__WarningBaselineRefreshI92__SelectNextBoundedCohort
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I92
Date: 2026-09-21
Parent: mirbuilder-warning-parser-source-catalog-same-parser-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded warning cohort only
NextCard: MIRBUILDER-WARNING-CALLABLE-CONTRACT-DISPOSITION-SYNTAX-DELETE-I0
---

# MirBuilder warning baseline refresh I92

## Six-line brief

```text
Decision: refresh warning surfaces after the parser source-catalog facade
  and select one finite caller-zero cohort only if compile-proven.
Source authority + canonical issuer: quick-profile Cargo diagnostics and the
  checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or an
  unclassified focused red.
Fail-fast boundary: a new warning, non-test caller, command drift, or red
  stops selection and records NoSafeSlice.
Smallest next slice: compare lib/lib-test against 1,712/553, census one
  candidate, and select Delete or NoSafeSlice.
Non-claims: no semantic refactor, suppression, production switch, or broad
  warning cleanup.
```

## Acceptance

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record warning class, file and line, owner, role, grouped-diagnostic
membership, and caller inventory. Select at most one caller-zero warning
facade whose production edge can be removed with a focused guard; otherwise
record `NoSafeSlice`. Keep dead-code and private-interface rows with their
owners. A selected edge must prove caller-zero before physical removal, then
run its focused gate and the stable warning refresh at its parent/current pair.

## I92 handoff evidence

The parser source-catalog facade closed with lib **1,712**, lib-test **553**,
and parser source tests **56/56**. This card remeasures that pair before
choosing the next warning row; it does not reopen parser source ownership.

## Refresh result and selected cohort

The sequential refresh completed with lib **1,712** warnings and lib-test
**553** warnings. No new red or private-interface diagnostic appeared.

The inventory retains `admitted_registry::branch_count` because
`aot_admission.rs:153` is a production caller, and retains the mixed-cfg
`loop_phi_materializer.rs:467` index warning for a separate cfg-aware row.
The next candidate is `src/parser/callable_contract_syntax.rs:42`,
`CallableContractSourceDispositionV1::syntax()`: repository-wide search found
no caller beyond its definition. Production consumers already pattern-match
the disposition directly, so deleting this unused accessor removes one dead
edge without introducing a replacement authority.

The next bounded cohort is
`MIRBUILDER-WARNING-CALLABLE-CONTRACT-DISPOSITION-SYNTAX-DELETE-I0`.
