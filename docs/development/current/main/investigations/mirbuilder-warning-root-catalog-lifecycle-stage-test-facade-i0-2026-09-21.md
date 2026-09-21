---
Status: closed__2026-09-21__WarningRootCatalogLifecycleStageTestFacade
Task: MIRBUILDER-WARNING-ROOT-CATALOG-LIFECYCLE-STAGE-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i30-2026-09-21.md
Implementation permission: true for one cfg(test) lifecycle-stage re-export only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I31
---

# Warning cleanup: root-catalog lifecycle stage test facade

## Six-line brief

```text
Decision: gate the lifecycle stage re-export consumed only by lifecycle tests;
  keep the normal_default_root_catalog_lifecycle owner unchanged.
Source authority + canonical issuer: normal_default_root_catalog_lifecycle.rs;
  cfg(test) export scope is the only moved boundary.
Non-authority: cargo-fix, wildcard imports, lifecycle semantics, visibility
  redesign, production route changes, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to the builder re-export and run the fixed
  quick-profile gates once each.
Non-claims: no lifecycle redesign, suppression, or production route change.
```

## Preconditions and acceptance

I30 records one lib-only unused-import diagnostic at
`src/mir/builder.rs:342`. The re-export is consumed by
`normal_default_root_catalog_lifecycle_tests.rs`; the lifecycle owner does not
need this builder-level re-export in production.

Acceptance requires lib warnings to drop from **1,801 to 1,800**, lib-test
warnings to remain **561**, and both fixed commands to exit 0:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
```

Only the re-export declaration may change. If the consumer classification or
warning count differs, stop and return to design stop.

## Execution evidence

The lifecycle-stage re-export is now gated with `#[cfg(test)]`. The fixed
sequential gates exited 0:

* `cargo check --profile quick --lib -j4`: lib warnings **1,800**.
* `cargo test --profile quick --lib --no-run -j4`: lib-test warnings **561**;
  the test executable was produced.

No production lifecycle state transition, lowering route, or owner module
changed.
