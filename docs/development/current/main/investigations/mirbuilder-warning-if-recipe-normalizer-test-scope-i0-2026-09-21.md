---
Status: closeout__2026-09-21__Landed__FocusedSuitesGreen
Task: MIRBUILDER-WARNING-IF-RECIPE-NORMALIZER-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i74-2026-09-21.md
Implementation permission: true for cfg(test) scoping of the two If recipe normalizer re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I75
---

# MirBuilder If recipe normalizer test scope I0

## Six-line brief

```text
Decision: remove the production re-export edge for the caller-zero If recipe
  normalizer pair while retaining existing test facades.
Source authority + canonical issuer: if_recipe_contract/normalize.rs.
Non-authority: parent-module reachability, warning counts, or a new recipe
  normalizer owner.
Fail-fast boundary: any non-test caller, missing test import, changed owner, or
  focused guard failure stops the slice.
Smallest next slice: add cfg(test) to the parent re-export and run If recipe
  and recipe-call tests plus the quick library check.
Non-claims: no normalizer semantic change, no verifier change, no production
  route change, no test deletion, and no blanket warning cleanup.
```

## Census and acceptance

`IfRecipeDecodeErrorV1` and `IfRecipeNormalizerV1` are defined in `normalize.rs`.
All imports through the parent facade are in test modules; production callers
use the owner module or do not call these helpers. Keep the existing test
imports and scope the parent re-export to `cfg(test)`.

Run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib if_recipe -- --nocapture
cargo test --profile quick --lib recipe_call -- --nocapture
```

Both focused suites must pass. Record the warning delta, preserve known
baseline classification, and run `cargo fmt --check`, `git diff --check`, and
the current-state pointer guard before commit/push.

## Closeout

The parent re-export now keeps `IfRecipeDecodeErrorV1` and
`IfRecipeNormalizerV1` only for test builds. The normalize owner, verifier, and
recipe semantics are unchanged. `cargo check --profile quick --lib -j4` passed
with **1,747** lib warnings (previously 1,748). The focused `if_recipe` suite
passed **24/24** and the `recipe_call` suite passed **6/6**; lib-test remained
at **557** warnings. `cargo fmt --check`, `git diff --check`, and the pointer
guard passed. No test was deleted, no warning was suppressed, and no production
route changed.
