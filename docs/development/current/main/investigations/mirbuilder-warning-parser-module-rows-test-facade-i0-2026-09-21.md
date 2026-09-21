---
Status: closed__2026-09-21__WarningParserModuleRowsTestFacade
Task: MIRBUILDER-WARNING-PARSER-MODULE-ROWS-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i14-2026-09-21.md
Implementation permission: true for one cfg(test) re-export gate only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I15
---

# Warning cleanup: parser module-rows test facade

## Six-line brief

```text
Decision: gate the parser module-rows re-export with cfg(test); the normal
  root test terminal keeps using the same disposition type.
Source authority + canonical issuer: Rust cfg/name resolution in
  script_source_authority/mod.rs, the module-rows owner, and the I14 warning
  inventory; production source authority keeps the defining module.
Non-authority: cargo-fix, wildcard imports, parser source semantics, visibility
  edits, dead-code ownership, or warning-count guesses.
Fail-fast boundary: any production consumer, compile error, new warning, or
  changed failure name rejects the move.
Smallest next slice: add cfg(test) to the selected re-export, then run both
  fixed checks and fmt/diff/pointer/lifecycle guards.
Non-claims: no parser source redesign, test behavior change, suppression,
  broad warning cleanup, or production route change.
```

## Precondition and acceptance

The I14 inventory records one lib-only unused-import diagnostic at
`src/parser/callable_parameter_source/script_source_authority/mod.rs:24`.
The re-export is consumed only by the `#[cfg(test)]` normal-root test
terminal; production uses the defining module directly. Acceptance requires
lib warnings to drop from 1,817 to 1,816, lib-test to remain 561, both fixed
quick-profile commands to exit 0, and fmt/diff/pointer/lifecycle guards to
remain green.

## Execution evidence

The selected re-export is now gated with `#[cfg(test)]`. `cargo check
--profile quick --lib -j4` exited 0 with lib warnings at 1,816, and `cargo
test --profile quick --lib --no-run -j4` exited 0 with lib-test warnings at
561 and produced the test executable. Production parser source authority
continues to use the defining module directly.
