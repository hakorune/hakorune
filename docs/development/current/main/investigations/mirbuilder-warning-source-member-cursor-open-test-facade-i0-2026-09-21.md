---
Status: closed__2026-09-21__WarningSourceMemberCursorOpenTestFacade
Task: MIRBUILDER-WARNING-SOURCE-MEMBER-CURSOR-OPEN-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i96-2026-09-21.md
Implementation permission: true for the test-only cursor constructor
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I97
---

# Warning cleanup: source member cursor test facade

## Six-line brief

```text
Decision: gate ParserBoxMemberSourceCursorV1::open() to tests; preserve the
production open_with_path constructor and all source-path semantics.
Source authority + canonical issuer: src/parser/source_member_cursor.rs and
the I96 quick-profile warning census.
Non-authority: open_with_path, source-path identity, parser semantics, warning
suppression, cargo-fix, or a guessed external/source caller.
Fail-fast boundary: any production caller, focused red, or warning-count
mismatch rejects the facade gating.
Smallest next slice: add cfg(test) to open(), run the cursor focused tests,
and refresh the fixed warning baseline.
Non-claims: no parser source expansion, route switch, old-edge deletion, or
semantic receipt change.
```

## Caller census

I96 records lib **1,708** and lib-test **552**. The exact
`ParserBoxMemberSourceCursorV1::open()` constructor is called by the
test-only `OpenBoxMethodSourceTransactionV1::open()` helper in
`source_authority.rs` and by the two cursor tests in
`source_member_cursor_tests.rs`. No production caller exists. Production
parsing uses `open_with_path()` and remains in scope for the existing source
authority path.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::source_member_cursor::tests
```

The focused filter must execute nonzero tests and pass. The stable refresh must
reduce lib warnings from **1,708** to **1,707**, keep lib-test at **552**, and
preserve the cursor identity/branch assertions. Run fmt, diff, and the
current-state pointer guard before closeout. Do not add an allow or alter
source-path semantics.

## Execution evidence

The test-only `open()` constructor is now gated with `cfg(test)`; production
`open_with_path()` and source-path identity construction are unchanged.
Sequential acceptance passed: lib check **1,707** warnings, lib-test build
**552**, and the cursor filter **3/3**. Fmt, diff, and the current-state
pointer guard passed.
