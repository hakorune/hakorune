---
Status: closed__2026-09-21__WarningSourceAuthorityTestFacade
Task: MIRBUILDER-WARNING-SOURCE-AUTHORITY-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i16-2026-09-21.md
Implementation permission: true for two cfg(test) parser source-authority re-exports only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I17
---

# Warning cleanup: parser source-authority test facade

## Six-line brief

```text
Decision: gate the two parser source-authority re-exports that are consumed
  only by parser tests; keep the defining modules and all production paths.
Source authority + canonical issuer: constructor_source.rs and source_path.rs;
  cfg(test) name resolution remains the only moved boundary.
Non-authority: cargo-fix, wildcard imports, visibility redesign, source-path
  semantics, constructor semantics, dead-code ownership, or warning guesses.
Fail-fast boundary: any production consumer, compile error, changed test
  warning, or warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to ConstructorSourceOriginV1 and
  SourceBoxPathSegmentV1 re-exports, then run the fixed gates once each.
Non-claims: no parser redesign, semantic change, suppression, or production
  route change.
```

## Precondition and acceptance

I16 records two lib-only unused-import diagnostics at
`src/parser/source_authority.rs:30` and `:64`. Each type is consumed through
the facade by existing `#[cfg(test)]` parser modules, while production code
imports the defining modules directly. Acceptance requires lib warnings to
drop from 1,816 to 1,814, lib-test warnings to remain 561, both fixed
quick-profile commands to exit 0, and fmt/diff/pointer/lifecycle guards to
remain green.

Only the two re-export declarations may change. If either count or consumer
classification differs, stop and return to design stop.

## Execution evidence

The two selected re-exports are now gated with `#[cfg(test)]`; defining
modules and production imports are unchanged. `cargo check --profile quick
--lib -j4` exited 0 with lib warnings at **1,814**. The sequential
`cargo test --profile quick --lib --no-run -j4` exited 0 with lib-test
warnings at **561** and produced the test executable. No semantic or
production-route change was introduced.
