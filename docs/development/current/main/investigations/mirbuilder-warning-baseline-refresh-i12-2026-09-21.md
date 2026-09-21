---
Status: closed__2026-09-21__WarningBaselineRefreshI12__SnapshotWitnessTestFacade
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I12
Date: 2026-09-21
Parent: mirbuilder-warning-invocation-identity-test-facade-i0-2026-09-21.md
Implementation permission: true for one cfg(test) snapshot-witness re-export move only
NextCard: MIRBUILDER-WARNING-SNAPSHOT-WITNESS-TEST-FACADE-I0
---

# MirBuilder warning baseline refresh I12

## Six-line brief

```text
Decision: refresh both warning surfaces after the invocation-identity
  test-facade cohort before selecting another cleanup row.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: historical counts, cargo-fix, blanket allow, visibility edits,
  dead-code guesses, or semantic interpretation of lint output.
Fail-fast boundary: command drift, new failure name, unclassified test-only
  reference, or a warning family outside the inventory stops selection.
Smallest next slice: compare current lib/lib-test inventories against
  1,819/561 and select at most one caller-zero import cohort.
Non-claims: no broad warning deletion, suppression, semantic refactor, or
  production route change.
```

## Acceptance

Run `cargo check --profile quick --lib -j4` and
`cargo test --profile quick --lib --no-run -j4` once each. Record lint,
file:line, owner, and production/test/compat/generated role, then select one
cohort or write `NoSafeSlice`. Keep dead-code/private-interface rows with
their owners. No code edit is permitted until the selection is recorded.

## I12 inventory and decision

The fresh commands both exited 0: lib produced 1,819 warnings and lib-test
produced 561 warnings. The `unused_imports` diagnostic at
`src/analysis/bounded_body_snapshot_v0/mod.rs:23` is one caller-zero
test-facade row for `build_snapshot_from_validated_view_v0`. Repository-wide
reference search finds only the analysis and strict-JSON snapshot test
modules; no production consumer imports this re-export. The selected next
slice was one `#[cfg(test)]` re-export gate, with expected lib 1,818 and
lib-test 561. Execution completed with those counts; the implementation card
records the fixed command evidence.
