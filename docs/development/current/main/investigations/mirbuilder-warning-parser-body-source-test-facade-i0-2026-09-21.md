---
Status: closed__2026-09-21__WarningParserBodySourceTestFacade
Task: MIRBUILDER-WARNING-PARSER-BODY-SOURCE-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i88-2026-09-21.md
Implementation permission: true for one cfg(test) parser body-source transaction facade scope only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I89
---

# Warning cleanup: parser body-source test facade

## Six-line brief

```text
Decision: gate the parser body-source transaction/error helpers and the
  release-source companion module with cfg(test); preserve the production
  envelope/row and syntax-lease surfaces.
Source authority + canonical issuer: src/parser/body_source.rs and
  src/parser/release_source.rs; repository-wide caller census and the I88
  quick-profile warning inventory define the boundary.
Non-authority: cargo-fix, wildcard imports, semantic parser changes, lease
  redesign, dead-code suppression, or warning-count guesses.
Fail-fast boundary: any production consumer of a gated item, compile error,
  focused test red, or warning-count mismatch rejects the move.
Smallest next slice: add cfg(test) only to the transaction/error support,
  its parser-only entry and release-source module/methods; keep the
  ParserBoxBodySourceEnvelopeV1, ParserBoxMethodBodySourceRowV1, and
  ParserBoxInstanceMethodSyntaxLeaseV1 paths unconditional.
Non-claims: no parser acceptance change, resolver behavior change, production
  route change, source authority change, or broad warning cleanup.
```

## Precondition and acceptance

I88 records lib **1,736** and lib-test **553**. The exact production caller
census has two preserved edges: `src/mir/resolved_semantics/
instance_method_function_carrier.rs` imports and consumes
`ParserBoxInstanceMethodSyntaxLeaseV1`, and
`src/mir/resolved_semantics/instance_method_body_source.rs` imports the body
envelope/row types. The selected transaction/error helpers are consumed by
`#[cfg(test)]` parser/resolved-semantics tests; `release_source` is reachable
only from the test transaction callback.

The slice may change only cfg registration and the corresponding test-only
entry/method boundaries. Acceptance requires:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::body_source -- --nocapture
```

The focused filter must execute a nonzero named test set. The stable refresh
must show fewer lib warnings, lib-test no regression, and no new private-
interface or compile diagnostics. Run fmt, diff, and the pointer guard before
closeout; do not add `#[allow]` or alter the production lease owner.

## Execution evidence

The selected transaction/error helpers and `release_source` module are now
test-only. The production envelope/row and syntax-lease surfaces remain
unconditional. `cargo check --profile quick --lib -j4` passed with **1,723**
lib warnings (13 fewer than I88); the sequential lib-test build passed with
**553** warnings. The focused body-source test passed **1/1**, the companion
release-source filter passed **7/7**, and the preserved function-carrier owner
passed **3/3**. No parser behavior or production route changed.

## Execution evidence

The selected transaction/error helpers and `release_source` module are now
test-only. The production envelope/row and syntax-lease surfaces remain
unconditional. `cargo check --profile quick --lib -j4` passed with **1,723**
lib warnings (13 fewer than I88); the sequential lib-test build passed with
**553** warnings. The focused body-source test passed **1/1**, the companion
release-source filter passed **7/7**, and the preserved function-carrier owner
passed **3/3**. No parser behavior or production route changed.
