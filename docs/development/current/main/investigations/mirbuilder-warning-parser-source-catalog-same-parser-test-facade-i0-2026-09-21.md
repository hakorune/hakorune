---
Status: closed__2026-09-21__WarningParserSourceCatalogSameParserTestFacade
Task: MIRBUILDER-WARNING-PARSER-SOURCE-CATALOG-SAME-PARSER-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i91-2026-09-21.md
Implementation permission: true for the test-only same_parser_source accessor gate only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I92
---

# Warning cleanup: parser source catalog same-parser test facade

## Six-line brief

```text
Decision: gate ParserCallableParameterSourceCatalogV1::same_parser_source
  to tests; preserve parser-brand comparison and all production catalog data.
Source authority + canonical issuer: src/parser/callable_parameter_source/
  catalog.rs and the I91 quick-profile warning/caller inventory.
Non-authority: cargo-fix, warning guesses, parser-source redesign, brand
  semantics, visibility changes, or production callers inferred from tests.
Fail-fast boundary: any production caller, compile error, focused red, or
  warning-count mismatch rejects the slice.
Smallest next slice: add cfg(test) to same_parser_source and run the parser
  source-catalog focused suite plus the fixed warning refresh.
Non-claims: no parser acceptance change, source-authority change, ABI change,
  suppression, or broad warning cleanup.
```

## Precondition and caller census

I91 records lib **1,713** and lib-test **553**. The selected diagnostic is the
unused method at `src/parser/callable_parameter_source/catalog.rs:36`.
Repository-wide search found exactly two callers, both test assertions in
`src/parser/callable_parameter_source/tests.rs:225-226`; no production module
calls this accessor. The catalog's brand, declarations, and production
`same_parser_brand` path remain unconditional.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::callable_parameter_source -- --nocapture
```

The focused filter must execute nonzero named tests and pass. The stable
refresh must reduce lib warnings from **1,713** to **1,712**, keep lib-test at
**553** with no new diagnostics, and preserve both same-parser assertions.
Run fmt, diff, and the current-state pointer guard before closeout. Do not add
`#[allow]`, delete a test, or alter parser source ownership.

## Execution evidence

`ParserCallableParameterSourceCatalogV1::same_parser_source` is now gated
with `cfg(test)`. The parser brand, declarations, and production
`same_parser_brand` relation remain unchanged.

Sequential acceptance passed:

* `cargo check --profile quick --lib -j4`: **1,712** lib warnings, one fewer
  than I91.
* `cargo test --profile quick --lib --no-run -j4`: **553** lib-test warnings.
* `cargo test --profile quick --lib parser::callable_parameter_source
  -- --nocapture`: **56/56**.
* `cargo fmt --all -- --check`, `git diff --check`, and the current-state
  pointer guard all passed.

No parser behavior, source ownership, test deletion, suppression, or
production caller changed. The next action is I92 warning baseline refresh.
