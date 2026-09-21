---
Status: closed__2026-09-21__WarningPostpassProgramCohortTestFacade
Task: MIRBUILDER-WARNING-POSTPASS-PROGRAM-COHORT-TEST-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i100-2026-09-21.md
Implementation permission: true for the test-only postpass observation accessor
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I101
---

# Warning cleanup: postpass program-cohort test facade

## Six-line brief

```text
Decision: gate ParserBoxPostpassCoverageV1::program_cohort() to tests;
preserve coverage rows, source-backed conversion, and postpass ownership.
Source authority + canonical issuer: src/parser/postpass_envelope.rs and the
I100 quick-profile warning census.
Non-authority: coverage rows, source conversion, parser semantics, warning
suppression, cargo-fix, or a guessed production caller.
Fail-fast boundary: any production caller, focused red, or warning-count
mismatch rejects the facade gating.
Smallest next slice: add cfg(test) to program_cohort(), run the postpass
envelope focused tests, and refresh the fixed warning baseline.
Non-claims: no parser source expansion, route switch, old-edge deletion, or
semantic receipt change.
```

## Caller census

I100 records lib **1,706** and lib-test **552**. The exact
`ParserBoxPostpassCoverageV1::program_cohort()` accessor has three callers in
the `postpass_envelope` test module and no production caller. The `rows()` and
`into_source_backed_ordinary_coverage()` methods remain production-visible and
are outside this row.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::postpass_envelope::tests
```

The focused filter must execute nonzero tests and pass. The stable refresh must
reduce lib warnings from **1,706** to **1,705**, keep lib-test at **552**, and
preserve the postpass cohort assertions. Run fmt, diff, and the current-state
pointer guard before closeout. Do not add an allow or alter postpass semantics.

## Execution evidence

The test-only `program_cohort()` accessor is now gated with `cfg(test)`;
postpass coverage rows and source-backed conversion remain unchanged.
Sequential acceptance passed: lib check **1,705** warnings, lib-test build
**552**, and the postpass envelope filter **7/7**. Fmt, diff, and the
current-state pointer guard passed.
