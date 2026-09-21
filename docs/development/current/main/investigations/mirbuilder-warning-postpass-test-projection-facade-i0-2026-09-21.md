---
Status: selected__2026-09-21__WarningPostpassTestProjectionFacade
Task: MIRBUILDER-WARNING-POSTPASS-TEST-PROJECTION-FACADE-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i102-2026-09-21.md
Implementation permission: true for the test-only postpass projection accessors
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I103
---

# Warning cleanup: postpass test projection facade

## Six-line brief

```text
Decision: gate ParserPostpassEnvelopeV1::metadata(), explain(), and
initial_callable_source() to tests; preserve consuming production projections.
Source authority + canonical issuer: src/parser/postpass_envelope.rs and the
I102 quick-profile warning census.
Non-authority: into_ast_and_metadata, into_ast_and_explain, source coverage,
parser semantics, warning suppression, cargo-fix, or guessed callers.
Fail-fast boundary: any production caller, focused red, or warning-count
mismatch rejects the facade gating.
Smallest next slice: add cfg(test) to the three accessors, run their focused
test modules, and refresh the fixed warning baseline.
Non-claims: no parser source expansion, route switch, old-edge deletion, or
semantic receipt change.
```

## Caller census

I102 records lib **1,705** and lib-test **552**. `metadata()` is called by
`source_seal_misc_tests.rs` and postpass tests; `explain()` is called by the
postpass and build-gate projection tests; `initial_callable_source()` is called
by `initial_callable_program_source/tests.rs`. All observed callers are
test-only. Production code consumes metadata/explain through the existing
`into_ast_and_metadata` and `into_ast_and_explain` terminals.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::postpass_envelope::tests
cargo test --profile quick --lib parser::build_cfg::projection::projection_tests
cargo test --profile quick --lib parser::initial_callable_program_source::tests
```

The focused filters must execute nonzero tests and pass. The stable refresh must
reduce lib warnings from **1,705** to **1,704**, keep lib-test at **552**, and
preserve the postpass, projection, and initial-source assertions. Run fmt,
diff, and the current-state pointer guard before closeout. Do not add an allow
or alter consuming projection semantics.

## Execution evidence

The three test-only accessors are now gated with `cfg(test)`; consuming
production projections remain unchanged. The initially attempted abbreviated
projection filter selected **0 tests** and was rejected as a filter error. The
registered module path was then used successfully. Sequential acceptance
passed: lib check **1,704** warnings, lib-test build **552**, postpass
envelope **7/7**, build-gate projection **3/3**, and initial-source tests
**6/6**. Fmt, diff, and the current-state pointer guard passed.
