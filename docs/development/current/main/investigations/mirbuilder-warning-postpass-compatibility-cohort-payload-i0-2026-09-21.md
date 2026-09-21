---
Status: closed__2026-09-21__WarningPostpassCompatibilityCohortPayload
Task: MIRBUILDER-WARNING-POSTPASS-COMPATIBILITY-COHORT-PAYLOAD-I0
Date: 2026-09-21
Parent: mirbuilder-warning-baseline-refresh-i103-2026-09-21.md
Implementation permission: true for the unread AST-only compatibility payload
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I104
---

# Warning cleanup: postpass compatibility cohort payload

## Six-line brief

```text
Decision: delete the unread AstOnlyCompatibility cohort payload and its
private conversion/helper plumbing; retain row presence and program_cohort as
the existing compatibility authority.
Source authority + canonical issuer: src/parser/postpass_envelope.rs and
postpass_envelope/source_rows.rs, selected from the I103 quick-profile census.
Non-authority: test-only row payload assertions, warning suppression,
cargo-fix, or a replacement compatibility classifier.
Fail-fast boundary: any production read, changed row cardinality, focused red,
or warning-count mismatch rejects the deletion.
Smallest next slice: remove the enum, conversion method, field, and unused
source-row arguments; keep the compatibility and mixed-program row guards.
Non-claims: no parser source expansion, route switch, publication, fallback,
old-edge deletion, or LegacyCallV0 retirement.
```

## Caller census

I103 records lib **1,704** and lib-test **552**. `ParserCompatibilityCohortV1`
has no caller outside the `AstOnlyCompatibility` field and its construction
arguments. The field has one test-only structural pattern at
`postpass_envelope.rs`; production row consumers match the variant without
reading a cohort. `ParserPostpassProgramCohortV1` remains used for program
classification and the named `SourceSealForCompatibility` error.

## Acceptance

Run sequentially with one Cargo process:

```text
cargo check --profile quick --lib -j4
cargo test --profile quick --lib --no-run -j4
cargo test --profile quick --lib parser::postpass_envelope::tests
```

The focused filter must execute nonzero tests and pass. The stable refresh must
reduce lib warnings from **1,704** to **1,703**, keep lib-test at **552**, and
preserve ordinary, compatibility, and mixed-program row assertions. Run fmt,
diff, and the current-state pointer guard before closeout. Do not add an allow
or alter parser semantics.

## Execution evidence

The unread `AstOnlyCompatibility` cohort payload, its private helper enum and
conversion, and the two source-row plumbing arguments were deleted. The row
variant, row cardinality, `program_cohort`, and named compatibility error remain
unchanged. Sequential acceptance passed: lib check **1,703** warnings,
lib-test build **552**, and the postpass envelope filter **7/7**. No warning
suppression was added; fmt and diff checks passed.
