---
Status: design_stop__2026-09-21__WarningBaselineRefreshI72__NextDeletionSelection
Task: MIRBUILDER-WARNING-BASELINE-REFRESH-I72
Date: 2026-09-21
Parent: mirbuilder-warning-static-method-cframe-test-scope-i0-2026-09-21.md
Implementation permission: false; refresh diagnostics and select one bounded deletion cohort only
NextCard: select one caller-zero warning facade or record NoSafeSlice
---

# MirBuilder warning baseline refresh I72

## Six-line brief

```text
Decision: refresh both warning surfaces after the C-frame cfg(test) scope and
  select one finite caller-zero source deletion if its owner is proven.
Source authority + canonical issuer: fixed quick-profile Cargo diagnostics and
  the checked-in warning classification policy.
Non-authority: warning-count guesses, cargo-fix, blanket allow, or test-only
  red interpretation without parent reproduction.
Fail-fast boundary: a new warning, a non-test caller, command drift, or an
  unclassified focused red stops deletion selection.
Smallest next slice: compare lib/lib-test against 1,750/557, census one
  caller-zero facade, and choose Delete or NoSafeSlice.
Non-claims: no semantic refactor, suppression, production switch, or broad
  warning cleanup.
```

## Acceptance

Run these commands once each, sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Record lint, file and line, owner, role, and grouped-diagnostic membership.
Select at most one caller-zero warning facade whose production edge can be
removed with a focused guard; otherwise record `NoSafeSlice`. Keep dead-code
and private-interface rows with their owners. The next selected deletion must
prove caller-zero before physical removal, then run its focused gate and the
stable warning refresh at its parent/current pair.

## I71/C-frame closeout evidence

The predecessor C-frame slice landed at commit `1067d0dc23`. It scopes
`PublishedStaticMethodCFrameV1` re-exports to `cfg(test)` in the function facade,
historical child facade, and normal compiler published-view facade. The C-frame
definition, fields, ABI, and test consumers are unchanged. Sequential quick
checks reported lib **1,750** warnings and lib-test **557** warnings.

The focused `published_backend_view` suite reported **88 passed, 5 failed,
1 ignored** at the current commit. The same five failures reproduced at the
parent commit, so they are `known baseline debt`, not current-change failures:
MapLiteral/New qualified-preflight failures, the existing typed assertion
mismatch, and the LLVM opaque-pointer/object failures. No test was deleted and
no warning was suppressed.
