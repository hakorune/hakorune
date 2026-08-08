---
Status: accepted design stop; implementation not opened
Date: 2026-08-09
Decision: broad AST-only parser APIs need a total typed postpass envelope before caller cutover
Parent: `frontend-parsed-box-source-aware-delegate-r6-s3b-d-i0-implementation-task-2026-08-09.md`
---

# PARSER-PUBLIC-AST-POSTPASS-CUTOVER-D0

## Decision

R6-S3B-D-I0 closes on the bounded rich path only:

```text
parse_from_string_with_source_seal
  -> ordinary top-level Box cohort
  -> C-I0 source-aware delegate batch
  -> complete generated relation/placement coverage
  -> sole ParserBoxSourceSealV1
```

The rich path contains zero direct callers of
`delegate_lowering::lower_delegate_exposes`. Its finalizer, focused tests,
guard, and documentation are the complete D-I0 receipt.

The following public AST-only APIs remain compatibility callers and are
explicit D-I0 nonclaims:

```text
NyashParser::parse
NyashParser::parse_from_string_with_fuel_and_build_config
NyashParser::parse_from_string_with_fuel_and_build_config_and_explain_report
```

They currently support cohorts that the bounded rich finalizer deliberately
rejects (`interface`, `static`, `record`, mixed programs, and selected gate
forms), and they preserve fuel, metadata, and explain-report behavior. A
direct replacement with the ordinary-only rich path would either break those
contracts or create a forbidden fallback. The D0 design permits no catch-and-fallback
path and no fake seal.

## Required design before implementation

Define one total, typed postpass envelope for broad AST parsing. It must make
the cohort boundary explicit rather than pretending every program has a source
seal:

```text
ParsedProgramPostpassEnvelopeV1
  = SupportedSourceSealed(ParsedProgramWithSourceV1)
  | CompatibilityAstOnly(typed cohort receipt + projected AST)
```

The exact public shape may change during D0, but the following facts are
mandatory:

```text
one parser invocation/session owner
one prune/delegate postpass owner
no AST/name rescan
no source identity reconstruction from inventory ordinals
no per-cohort retry or fallback after selection
ordinary source seals are consumed only for supported cohorts
compatibility cohorts never become resolver authority
fuel behavior is preserved
ParserMetadata behavior is preserved
BuildGateExplainReport is produced at its existing pre-prune boundary
NyashParser::parse uses the same postpass owner
```

## Required census and acceptance matrix

Before I0, enumerate every production and test caller of the three public
APIs and classify its source cohort:

```text
ordinary top-level Box
interface/static/record Box
mixed program
top-level build gate
metadata consumer
explain-report consumer
```

The D0 design must specify the expected envelope variant and diagnostic for
each cohort. Positive parity must compare AST shape and diagnostics; it must
not compare source-seal internals for compatibility variants.

## Stop lines

Do not implement this row by:

```text
calling parse_from_string_with_source_seal and catching its rejection
trying rich then legacy
adding a name/cohort fallback in delegate lowering
silently dropping fuel, metadata, or explain reports
issuing an empty/fake ParserBoxSourceSealV1
making interface/static/record rows look ordinary
```

The implementation row opens only after the total envelope, owner table,
source/reference parity matrix, and fail-fast diagnostics are accepted.
Implementation must update the parser README, language reference, task map,
CURRENT_STATE, focused tests, and a row guard in the same slice. All touched
source files remain below 800 lines.
