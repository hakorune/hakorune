---
Status: design_stop__2026-09-21__WarningBaselineRefreshI69__NextCohortSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I69
Date: 2026-09-21
Parent: mirbuilder-warning-dynamic-invocation-test-facade-i0-2026-09-21.md
Implementation permission: false; refresh and select one cohort only
NextCard: MIRBUILDER-WARNING-SURFACE-CENSUS-R0
---

# MirBuilder warning baseline refresh I69

## Six-line brief

```text
Decision: refresh both warning surfaces after the dynamic-invocation facade
  deletion and focused tests.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: diagnostic guesses, cargo-fix, blanket allow, or dead-code
  interpretation without a caller census.
Fail-fast boundary: a new warning, owner-local move, command drift, or an
  unclassified test-only reference stops selection.
Smallest next slice: compare lib/lib-test against 1,753/557 and select at most
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
generated **1,753** warnings and lib-test generated **557** warnings. The next
caller-zero cohort is the test-only portion of
`src/mir/dynamic_operator_contract/mod.rs`: the issuer function and issue enum,
plus the six model enums `DynamicOperatorControlV1`,
`DynamicOperatorEffectV1`, `DynamicOperatorFaultV1`,
`DynamicOperatorInputAccessV1`, `DynamicOperatorOrderingV1`, and
`DynamicOperatorSuspensionV1`. A complete census finds these references only
in `issuer.rs`, `model.rs`, and `tests.rs`; the parent re-export itself has no
production consumer. The bounded slice moves the test imports directly into
`tests.rs` and leaves the used domain/family/result/value exports untouched.

Selected successor: `MIRBUILDER-WARNING-DYNAMIC-OPERATOR-TEST-FACADE-I0`.
No operator semantics, issuer behavior, or route is in scope.
