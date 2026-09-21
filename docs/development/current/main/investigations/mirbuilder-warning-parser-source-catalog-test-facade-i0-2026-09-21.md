---
Status: closed__2026-09-21__WarningParserSourceCatalogTestFacade
Task: MIRBUILDER-WARNING-PARSER-SOURCE-CATALOG-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i13-2026-09-21.md
Implementation permission: true for one cfg(test) import split only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I14
---

# Warning cleanup: parser source catalog test facade

## Six-line brief

```text
Decision: split the parser source catalog import and gate its test-only type
  with cfg(test); production keeps the disposition import unchanged.
Source authority + canonical issuer: Rust cfg/name resolution in
  callable_parameter_source/product.rs, its terminal helpers, and the I13
  warning inventory; the catalog owner remains canonical.
Non-authority: cargo-fix, wildcard imports, parser source semantics, visibility
  edits, dead-code ownership, or warning-count guesses.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: separate the selected type import, add cfg(test), then
  run both fixed checks and fmt/diff/pointer/lifecycle guards.
Non-claims: no parser source redesign, test behavior change, suppression,
  broad warning cleanup, or production route change.
```

## Precondition and acceptance

The I13 inventory records one lib-only unused-import diagnostic at
`src/parser/callable_parameter_source/product.rs:3`. The catalog type is
consumed only by `#[cfg(test)]` terminal helpers; production retains the
existing disposition and catalog owners. Acceptance requires lib warnings to
drop from 1,818 to 1,817, lib-test to remain 561, both fixed quick-profile
commands to exit 0, and fmt/diff/pointer/lifecycle guards to remain green.

## Execution evidence

The selected catalog import is now split and the test-only type is gated with
`#[cfg(test)]`. `cargo check --profile quick --lib -j4` exited 0 with lib
warnings at 1,817, and `cargo test --profile quick --lib --no-run -j4` exited
0 with lib-test warnings at 561 and produced the test executable. Production
parser source disposition and catalog owners remain unchanged.
