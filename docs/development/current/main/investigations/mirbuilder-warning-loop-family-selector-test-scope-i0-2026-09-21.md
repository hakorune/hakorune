---
Status: fast__2026-09-21__SelectedBoundedWarningCohort__ExecuteTestScope
Task: MIRBUILDER-WARNING-LOOP-FAMILY-SELECTOR-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i76-2026-09-21.md
Implementation permission: true for cfg(test) scoping of the three canonical loop selector re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I77
---

# MirBuilder loop family selector test scope I0

## Six-line brief

```text
Decision: remove the production re-export edge for three caller-zero canonical
  loop selector symbols while retaining the existing test facade.
Source authority + canonical issuer: loop_route_policy/family_selector.rs.
Non-authority: parent-module reachability, warning counts, or a new selector
  owner.
Fail-fast boundary: any non-test caller, missing test import, changed owner, or
  focused guard failure stops the slice.
Smallest next slice: add cfg(test) to the parent re-export and run the selector
  suite plus the quick library check.
Non-claims: no selector semantic change, no admission change, no production
  route change, no test deletion, and no blanket warning cleanup.
```

## Census and acceptance

`select_canonical_loop_family_v1`, `CanonicalLoopFamilySelectionOutcomeV1`,
and `CanonicalLoopFamilySelectionReasonV1` are defined in
`family_selector.rs`. All parent-facade consumers are in
`family_selector_tests.rs`; no non-test caller exists. Keep the tests and scope
the parent re-export to `cfg(test)`.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib family_selector -- --nocapture
```

The focused suite must pass. Record the warning delta, preserve known baseline
classification, and run `cargo fmt --check`, `git diff --check`, and the
current-state pointer guard before commit/push.
