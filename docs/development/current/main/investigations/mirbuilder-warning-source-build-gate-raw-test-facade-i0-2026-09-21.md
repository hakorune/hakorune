---
Status: closed__2026-09-21__WarningSourceBuildGateRawTestFacade
Task: MIRBUILDER-WARNING-SOURCE-BUILD-GATE-RAW-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i94-2026-09-21.md
Implementation permission: true for the test-only SourceBuildGateId::raw accessor
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I95
---

# Warning cleanup: SourceBuildGateId raw test facade

## Six-line brief

```text
Decision: gate SourceBuildGateIdV1::raw() to tests; preserve the source gate
identity and all production path construction/validation.
Source authority + canonical issuer: src/parser/source_path.rs and the I94
quick-profile warning census.
Non-authority: gate identity allocation, source-path structure, parser
semantics, warning suppression, or a new receipt.
Fail-fast boundary: any production caller, focused red, or warning-count
mismatch rejects the facade gating.
Smallest next slice: add cfg(test) to raw(), run source-session tests, and
refresh the fixed warning baseline.
Non-claims: no parser source expansion, route switch, old-edge deletion, or
ABI change.
```

## Caller census

I94 records lib **1,710** and lib-test **552**. `SourceBuildGateIdV1::raw()`
has five repository callers, all in `src/parser/source_session_tests.rs`:
two sibling-child path assertions, one nested path assertion, and two ledger
ordinal assertions. `rg` found no production caller. `from_raw()` and the
identity field remain production-visible within the parser; only the test
projection is gated.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::source_session_tests
```

The focused filter must execute nonzero tests and pass. The stable refresh
must reduce lib warnings from **1,710** to **1,709**, keep lib-test at **552**,
and preserve the five raw-coordinate assertions. Run fmt, diff, and the
current-state pointer guard before closeout. Do not add an allow or alter
source gate identity semantics.

## Execution evidence

The `raw()` accessor is now `cfg(test)` only; production gate identity
construction and path validation are unchanged. Sequential acceptance passed:
lib check **1,709** warnings, lib-test build **552**, and the focused source
session filter **6/6**. Fmt, diff, and the current-state pointer guard passed;
no source gate semantics or production route changed.
