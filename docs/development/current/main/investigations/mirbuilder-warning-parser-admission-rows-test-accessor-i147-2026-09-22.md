---
Status: closed__2026-09-22__WarningParserAdmissionRowsTestAccessorI147
Task: MIRBUILDER-WARNING-PARSER-ADMISSION-ROWS-TEST-ACCESSOR-I147
Date: 2026-09-22
Parent: mirbuilder-warning-baseline-refresh-i146-2026-09-22.md
Implementation permission: true; gate one test-only witness accessor
NextCard: MIRBUILDER-WARNING-BASELINE-REFRESH-I148
---

# MirBuilder parser admission rows test accessor I147

## Six-line brief

```text
Decision: keep witness rows observable to its focused tests while excluding
  the unused accessor from the production library surface.
Source authority + canonical issuer: ParserSourceAdmissionWitnessV1 issued by
  the existing parser admission owner; no semantic product is added.
Non-authority: test naming, AST/MIR inference, warning counts, or fallback.
Fail-fast boundary: any non-test caller or failed admission test reopens the
  design stop.
Smallest next slice: add cfg(test) to ParserSourceAdmissionWitnessV1::rows.
Non-claims: no admission behavior, source lineage, parser route, old-edge,
  backend, or production caller change.
```

## Census and delete set

`ParserSourceAdmissionWitnessV1::rows` is declared in
`src/parser/normal_callable_program_source/source_admission.rs`. Its only
callers are the focused test assertions that inspect canonical segment and
local line values. The production owner stores and transports the witness
without row projection. The delete set is exactly the declaration's
production compilation edge; no test or source admission code is removed.

## Acceptance

```text
cargo fmt --all -- --check
CARGO_BUILD_JOBS=4 cargo check --profile quick --lib
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_callable_program_source::source_admission
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib --no-run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Record the before/after lib and lib-test warning counts and the focused
admission test result.

## Closeout evidence

The `rows` declaration now carries `#[cfg(test)]`; the admission issuer and
all production consumers remain unchanged. The focused source-admission suite
passed **3/3**. Sequential quick checks produced:

```text
baseline lib check (I146): 1,673 warnings
post-change lib check:      1,672 warnings
post-change lib-test:         543 warnings
```

`cargo test --profile quick --lib --no-run` passed with the same 543-warning
lib-test baseline. Formatter, pointer guard, and diff checks passed. No
production caller of the accessor was found or introduced.
