---
Status: design_stop__2026-09-21__WarningBaselineRefreshI70__NextCohortSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I70
Date: 2026-09-21
Parent: mirbuilder-warning-dynamic-operator-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-SURFACE-CENSUS-R0
---

# MirBuilder warning baseline refresh I70

## Six-line brief

```text
Decision: refresh both warning surfaces after the dynamic-operator facade
  deletion and focused tests.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: diagnostic guesses, cargo-fix, blanket allow, or dead-code
  interpretation without a caller census.
Fail-fast boundary: a new warning, owner-local move, command drift, or an
  unclassified test-only reference stops selection.
Smallest next slice: compare lib/lib-test against 1,751/557 and select at most
  one caller-zero cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or route
  change.
```

## Acceptance

Run these commands once each, sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record lint, file and line, owner, role, and grouped-diagnostic membership.
Select one finite caller-zero import cohort or write `NoSafeSlice`. Keep
dead-code and private-interface rows with their owners.

## Refresh result and bounded selection

The sequential refresh completed on 2026-09-21 with no new failure: lib
generated **1,751** warnings and lib-test generated **557** warnings. The next
caller-zero cohort is the four test-only JoinSig exports at
`src/mir/if_recipe_contract/mod.rs:26-27`: `IfJoinEdgeV1`,
`IfJoinObligationV1`, `IfJoinSigElaboratorV1`, and `IfJoinValueEdgeV1`. A
complete census finds their definitions in `join_sig.rs` and uses only in the
if-contract tests and the test module nested in the physicalizer; no production
consumer uses the parent facade. The bounded slice moves direct `join_sig`
imports into those two test modules and removes only these four re-exports.

Selected successor: `MIRBUILDER-WARNING-IF-JOINSIG-TEST-FACADE-I0`.
The remaining JoinSig roles, physicalizer behavior, and If recipe semantics are
outside the slice.
