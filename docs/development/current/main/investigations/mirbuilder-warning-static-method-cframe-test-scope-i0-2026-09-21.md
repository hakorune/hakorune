---
Status: closeout__2026-09-21__Landed__ParentBaselineReproduced
Task: MIRBUILDER-WARNING-STATIC-METHOD-CFRAME-TEST-SCOPE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i71-2026-09-21.md
Implementation permission: true for CFrame cfg(test) scoping only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I72
---

# MirBuilder static method C-frame test scope I0

## Six-line brief

```text
Decision: scope the historical C-frame re-exports to cfg(test).
Source authority + canonical issuer: compiler published_backend_view/c_transport.
Non-authority: the function facade in production builds or warning-count guesses.
Fail-fast boundary: a non-test caller, missing test symbol, or changed C-frame
  layout stops the slice.
Smallest next slice: add cfg(test) to CFrame re-exports in function.rs and its
  historical child facade, then run the fixed quick gates.
Non-claims: no C-row change, no ABI/layout change, no test deletion, no route
  change, and no production semantic change.
```

## Evidence and acceptance

I71 refreshed the fixed baseline at lib **1,751** / lib-test **557**. The source
census is finite and all C-frame consumers are test-only. The fast slice must
run sequentially:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

The published-backend-view and MIR JSON test paths must still compile and pass;
the C-frame type and fields remain unchanged. No production import may be added.

## Closeout

Commit `1067d0dc23` scopes `PublishedStaticMethodCFrameV1` re-exports to
`cfg(test)` in the function facade, historical child facade, and normal compiler
published-view facade. The C-frame definition, fields, ABI, and test consumers
are unchanged. The sequential quick gates reported lib **1,750** warnings and
lib-test **557** warnings.

The focused `published_backend_view` suite reported **88 passed, 5 failed,
1 ignored** at the current commit, and the same five failures reproduced at the
parent commit. They are recorded as `known baseline debt`: MapLiteral/New
qualified-preflight failures, the existing typed assertion mismatch, and LLVM
opaque-pointer/object failures. No warning was suppressed and no test was
deleted. `cargo fmt --check`, `git diff --check`, and the current-state pointer
guard are green.
