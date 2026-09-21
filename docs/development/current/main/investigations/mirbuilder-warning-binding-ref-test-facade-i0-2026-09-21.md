---
Status: closed__2026-09-21__WarningBindingRefTestFacade
Task: MIRBUILDER-WARNING-BINDING-REF-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i28-2026-09-21.md
Implementation permission: true for one cfg(test) BindingRefV1 import only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I29
---

# Warning cleanup: Script semantic-source BindingRef test facade

## Six-line brief

```text
Decision: resolve BindingRefV1 only in the test-only outbox accessor; keep the
  Script semantic-source owner and its production imports unchanged.
Source authority + canonical issuer: normal_script_semantic_source.rs and its
  existing resolved-semantics types; cfg(test) import scope is the boundary.
Non-authority: cargo-fix, wildcard imports, resolver changes, semantic source
  redesign, visibility changes, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: move BindingRefV1 into the existing cfg(test) import and
  run the fixed quick-profile gates once each.
Non-claims: no source-product redesign, suppression, or production route change.
```

## Preconditions and acceptance

I28 records one lib-only unused-import diagnostic at
`src/mir/builder/normal_script_semantic_source.rs:20`. The import is consumed
only by the `#[cfg(test)]` `outbox_materializations()` accessor.

Acceptance requires lib warnings to drop from **1,803 to 1,802**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the import grouping may change. If the consumer classification or warning
count differs, stop and return to design stop.

## Execution evidence

`BindingRefV1` is now in the test-only resolved-semantics import group. The
fixed sequential gates exited 0:

* `cargo check --profile quick --lib -j4`: lib warnings **1,802**.
* `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **561**;
  the test executable was produced.

No production Script source consumer, resolver relation, or lowering route
changed.
