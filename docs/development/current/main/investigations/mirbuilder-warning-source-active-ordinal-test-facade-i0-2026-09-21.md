---
Status: closed__2026-09-21__WarningSourceActiveOrdinalTestFacade
Task: MIRBUILDER-WARNING-SOURCE-ACTIVE-ORDINAL-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i98-2026-09-21.md
Implementation permission: true for the test-only parser observation accessor
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I99
---

# Warning cleanup: source active-ordinal test facade

## Six-line brief

```text
Decision: gate NyashParser::active_source_statement_ordinal() to tests;
preserve parser source identity and all production source-path state.
Source authority + canonical issuer: src/parser/source_path.rs and the I98
quick-profile warning census.
Non-authority: active source declaration paths, source identity issuance,
parser semantics, warning suppression, cargo-fix, or a guessed caller.
Fail-fast boundary: any production caller, focused red, or warning-count
mismatch rejects the facade gating.
Smallest next slice: add cfg(test) to active_source_statement_ordinal(), run
the source-session focused tests, and refresh the fixed warning baseline.
Non-claims: no parser source expansion, route switch, old-edge deletion, or
semantic receipt change.
```

## Caller census

I98 records lib **1,707** and lib-test **552**. The exact
`NyashParser::active_source_statement_ordinal()` accessor has two callers in
`source_session_tests.rs`, both test-only. No production caller exists. The
neighboring `active_source_declaration_path()` accessor and source identity
fields remain production-visible and are outside this row.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::source_session_tests
```

The focused filter must execute nonzero tests and pass. The stable refresh must
reduce lib warnings from **1,707** to **1,706**, keep lib-test at **552**, and
preserve the source-session assertions. Run fmt, diff, and the current-state
pointer guard before closeout. Do not add an allow or alter source identity
semantics.

## Execution evidence

The test-only `active_source_statement_ordinal()` accessor is now gated with
`cfg(test)`; parser source identity issuance and the neighboring production
path accessor are unchanged. Sequential acceptance passed: lib check **1,706**
warnings, lib-test build **552**, and the source-session filter **6/6**. Fmt,
diff, and the current-state pointer guard passed.
