---
Status: closeout__2026-09-21__Landed__FocusedSuiteGreen
Task: MIRBUILDER-WARNING-IF-JOIN-REEXPORT-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i72-2026-09-21.md
Implementation permission: true for cfg(test) scoping of the three If Join re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I73
---

# MirBuilder If Join re-export test scope I0

## Six-line brief

```text
Decision: remove the production re-export edge for three caller-zero If Join
  types while retaining the existing test facade.
Source authority + canonical issuer: if_recipe_contract/join_sig.rs.
Non-authority: parent-module re-export reachability, warning counts, or a new
  JoinSig/physical owner.
Fail-fast boundary: any non-test caller, missing test import, changed type
  ownership, or focused guard failure stops the slice.
Smallest next slice: add cfg(test) to the parent re-export and run If recipe
  contract/physicalizer tests plus the quick library check.
Non-claims: no JoinSig semantic change, no physicalizer change, no production
  route change, no test deletion, and no blanket warning cleanup.
```

## Census and implementation scope

`IfJoinEdgeV1`, `IfJoinObligationV1`, and `IfJoinValueEdgeV1` are defined and
constructed in `join_sig.rs`. Production physicalizer code imports them through
that owner module. The only imports through `if_recipe_contract` parent are in
`#[cfg(test)]` physicalizer tests. The parent `pub(crate) use` is therefore a
caller-zero production facade edge; keep it for test builds and remove it from
normal library builds with `#[cfg(test)]`.

## Acceptance

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib if_recipe -- --nocapture
```

The focused suite must compile and pass. Record the warning count delta, retain
the I72 baseline red classification, and run `cargo fmt --check`, `git diff
--check`, and the current-state pointer guard before commit/push.

## Closeout

The parent re-export was split so `IfJoinEdgeV1`, `IfJoinObligationV1`, and
`IfJoinValueEdgeV1` remain available to test consumers under `cfg(test)` while
production loses the caller-zero facade edge. The canonical JoinSig definitions
and the physicalizer are unchanged.

`cargo check --profile quick --lib -j4` passed with **1,749** lib warnings
(previously 1,750). The focused `if_recipe` suite passed **24/24** with no
failures. `cargo fmt --check`, `git diff --check`, and the pointer guard passed.
No test was deleted, no warning was suppressed, and no semantic or production
route changed.
