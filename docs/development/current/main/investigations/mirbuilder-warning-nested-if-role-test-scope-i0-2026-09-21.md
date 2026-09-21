---
Status: closeout__2026-09-21__Landed__FocusedSuiteGreen
Task: MIRBUILDER-WARNING-NESTED-IF-ROLE-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i73-2026-09-21.md
Implementation permission: true for cfg(test) scoping of NestedIfJoinCompositionRoleV1 only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I74
---

# MirBuilder nested If role test scope I0

## Six-line brief

```text
Decision: remove the production re-export edge for the caller-zero nested If
  composition role while retaining its existing test facade.
Source authority + canonical issuer: if_recipe_contract/nested_join_sig.rs.
Non-authority: parent-module reachability, warning counts, or a new nested
  JoinSig owner.
Fail-fast boundary: any non-test caller, missing test import, changed owner, or
  focused guard failure stops the slice.
Smallest next slice: add cfg(test) to the parent re-export and run nested If
  recipe tests plus the quick library check.
Non-claims: no nested JoinSig semantic change, no physicalizer change, no
  production route change, no test deletion, and no blanket warning cleanup.
```

## Census and acceptance

`NestedIfJoinCompositionRoleV1` is defined in `nested_join_sig.rs`; its only
parent-facade consumer is the `resolved_value_profile` nested-recipe test. The
production graph has no caller. Keep that test import and scope the parent
re-export to `cfg(test)`.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib nested_recipe -- --nocapture
```

The focused suite must pass. Record the warning delta, preserve known baseline
classification, and run `cargo fmt --check`, `git diff --check`, and the
current-state pointer guard before commit/push.

## Closeout

The parent re-export now keeps `NestedIfJoinCompositionRoleV1` only for test
builds. Its canonical definition and nested JoinSig composer are unchanged.
`cargo check --profile quick --lib -j4` passed with **1,748** lib warnings
(previously 1,749), and the focused `nested_recipe` suite passed **4/4**.
`cargo fmt --check`, `git diff --check`, and the pointer guard passed. No test
was deleted, no warning was suppressed, and no production route changed.
