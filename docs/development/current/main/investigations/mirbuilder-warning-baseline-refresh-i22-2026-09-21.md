---
Status: closed__2026-09-21__WarningBaselineRefreshI22__SelectedInvocationCollectionTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I22
Date: 2026-09-21
Parent: mirbuilder-warning-instance-constructor-manifest-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) invocation-collection family import facade only
NextCard: MIRBUILDER-WARNING-INVOCATION-COLLECTION-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I22

## Six-line brief

```text
Decision: refresh both warning surfaces after the instance-constructor
  manifest test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,809/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I22 inventory and decision

The fixed commands completed successfully with the expected baseline:

| surface | warnings | evidence |
| --- | ---: | --- |
| lib | 1,809 | `/tmp/hakorune-warning-i22-lib-20260921.log` |
| lib test | 561 | `/tmp/hakorune-warning-i22-lib-test-20260921.log` |

The selected caller-zero cohort is the invocation-collection test facade:

* `src/mir/builder/module_invocation_collection.rs:21` —
  `ResolvedOwnerHeaderFamilyV1` is consumed only by the local `#[cfg(test)]`
  canonical-source helpers; production collection code uses the verified
  header without this family enum.

The bounded next slice is to gate only this family import with `#[cfg(test)]`;
invocation collection and header-family semantics remain unchanged.
