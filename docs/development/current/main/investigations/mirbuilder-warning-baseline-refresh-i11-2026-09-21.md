---
Status: closed__2026-09-21__WarningBaselineRefreshI11__InvocationIdentityTestFacadeSelected
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I11
Date: 2026-09-21
Parent: mirbuilder-warning-callable-batch-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) invocation-identity import move only
NextCard: MIRBUILDER-WARNING-INVOCATION-IDENTITY-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I11

## Six-line brief

```text
Decision: refresh both warning surfaces after the callable-batch test-facade
  cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,820/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I11 inventory and decision

The fresh commands both exited 0: lib produced 1,820 warnings and lib-test
produced 561 warnings. The `unused_imports` diagnostic at
`src/mir/builder/module_invocation_identity.rs:8` is one caller-zero
test-facade row for `std::num::NonZeroU64`: every reference is inside the
file's `#[cfg(test)] TestInvocationPreflightFactoryV1`, while the production
`ModuleInvocationTokenV1` issuer has no dependency on that import. The
selected next slice is one `#[cfg(test)]` import gate, with expected lib
1,819 and lib-test 561.
