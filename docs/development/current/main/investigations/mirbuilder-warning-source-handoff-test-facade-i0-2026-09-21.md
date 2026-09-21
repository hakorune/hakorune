---
Status: closed__2026-09-21__WarningSourceHandoffTestFacade
Task: MIRBUILDER-WARNING-SOURCE-HANDOFF-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i89-2026-09-21.md
Implementation permission: true for one cfg(test) source-resolver handoff test facade only
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I90
---

# Warning cleanup: source-resolver handoff test facade

## Six-line brief

```text
Decision: gate the source-resolver handoff builder/validation helpers and
  parser entry with cfg(test); preserve the AST-free handoff/site/provenance
  types consumed by production source carriers.
Source authority + canonical issuer: src/parser/source_resolver_handoff.rs
  and its exact #[cfg(test)] callers recorded by I89.
Non-authority: cargo-fix, wildcard imports, AST rewrite, source identity
  redesign, dead-code suppression, or warning-count guesses.
Fail-fast boundary: any production consumer of a gated helper, compile error,
  focused test red, or warning-count mismatch rejects the move.
Smallest next slice: gate only ResolverSourceHandoffErrorV1, the parser entry,
  into_ast handoff, builder, and private collection helpers; keep the
  ResolverBoxMethodSource* and provenance products unconditional.
Non-claims: no resolver behavior change, source authority change, production
  route change, or broad warning cleanup.
```

## Precondition and acceptance

I89 refresh records lib **1,723** and lib-test **553**. The selected helper
surface has test callers in parser/resolver test modules and in the already
test-only body-source transaction. Production modules import only the
AST-free `ParserBoxResolverSourceHandoffV1`, method-row/site, and invocation
provenance products; those definitions and their accessors remain available.

Only cfg registration and the selected helper boundaries may change.
Acceptance requires:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib source_resolver_handoff -- --nocapture
```

The focused filter must execute a nonzero named test set. The stable refresh
must show a warning reduction from 1,723, lib-test no regression, and no new
compile or private-interface diagnostic. Run fmt, diff, and the pointer guard
before closeout; do not add `#[allow]` or gate the production carrier types.

## Execution evidence

The parser entry, handoff builder/validation helpers, error enum, and neutral
parameter projection helper are now test-only. The AST-free handoff, row/site,
provenance, and parameter carrier types remain unconditional for production
consumers. `cargo check --profile quick --lib -j4` passed with **1,715** lib
warnings (8 fewer than I89); the sequential lib-test build passed with **553**
warnings. The source-handoff focused suite passed **3/3**, and the preserved
`instance_method_declaration` consumer suite passed **8/8**. No resolver
behavior or production route changed.
