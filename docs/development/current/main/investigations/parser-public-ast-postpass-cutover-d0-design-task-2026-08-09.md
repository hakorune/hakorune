---
Status: accepted design; implementation not opened
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

## D0 decision: one total envelope, two explicit authority variants

The design is now fixed after an independent top-down audit. The broad AST
surface must use one postpass owner, but AST transport and resolver-visible
source authority remain separate:

```text
parse_program once
  -> full BuildGate decision set
  -> one prune / delegate postpass coordinator
  -> one final cohort coverage pass
  -> CompletedParserPostpassV1
```

The private result is conceptually:

```text
CompletedParserPostpassV1 {
    ast: ASTNode,
    metadata: ParserMetadata,
    explain: Option<BuildGateExplainReport>,
    box_coverage: ParserBoxPostpassCoverageV1,
}

ParserBoxPostpassCoverageV1 {
    program_cohort: ParserPostpassProgramCohortV1,
    boxes: [ParserBoxPostpassRowV1],
}

ParserBoxPostpassRowV1
  = SourceSealedOrdinary {
        final_statement_placement,
        seal: ParserBoxSourceSealV1,
    }
  | AstOnlyCompatibility {
        final_statement_placement,
        cohort: ParserCompatibilityCohortV1,
    }
```

The names may remain private or be adjusted during implementation, but the
authority split is normative:

```text
SourceSealedOrdinary
  -> may be projected to resolver-visible ParserBoxSourceSealV1

AstOnlyCompatibility
  -> AST/metadata/explain projection only
  -> never a source seal, resolver target, Recipe input, or fake seal
```

`final_statement_placement` is an AST coverage coordinate, not declaration
identity. Source identity remains the parser-issued path/source-site product;
inventory ordinals and final AST positions are never resolver identity.

The sole owner is an `OpenParserPostpassProductV1`/private coordinator that
consumes the AST, parser source session, metadata, and explain demand exactly
once. Static string APIs and `NyashParser::parse` must both reach this owner;
the latter must not re-tokenize or call a string API again.

## Cohort selection and failure ownership

The coordinator performs one typed admission before choosing one postpass arm.
The admission is structural and source-backed; it does not classify by Box or
method name and it does not retry after a choice:

```text
OrdinaryTopLevelBox
  -> SourceSealedOrdinary when complete final relation/placement coverage exists

SelectedTopLevelBuildGate
  -> SourceSealedOrdinary only when the selected branch resolves to the same
     supported ordinary cohort and its original source path is retained
  -> otherwise an explicit compatibility/rejection decision, never rich-then-old

InterfaceBox / StaticBox / RecordBox / MixedProgram
  -> AstOnlyCompatibility with a typed cohort receipt
```

The full program cohort and each Box row are kept distinct so a mixed program
cannot be made to look like one ordinary Box. A compatibility row is a
successful AST contract, not `Declined`, and it never receives an empty seal.

The failure matrix is fixed as follows:

```text
syntax / token / fuel exhaustion
  -> existing ParseError family

foreign, duplicate, malformed, or missing source relation
  -> reject and drop the whole unpublished postpass product

source/cohort alignment unavailable
  -> Unresolved or explicit typed postpass diagnostic

ordinary source seal requested for AstOnlyCompatibility
  -> typed IncompleteSourceAuthorityCoverage

compatibility cohort selected
  -> no fallback to the ordinary arm and no retry of the old whole-root helper
```

`NoSafeSlice` remains a development state for an unimplemented coordinator or
issuer; it is not converted into `Candidate`, `Declined`, `Unresolved`, or
`Rejected` merely to open implementation early.

## Build-gate, fuel, metadata, and explain contracts

The existing explain path recursively observes all BuildGate nodes, while the
current source ledger is top-level scoped. D0 therefore requires a private
`PreparedBuildGateDecisionSetV1` (name provisional) that evaluates every
structural BuildGate path once. The same decision set is consumed by:

```text
(a) AST pruning,
(b) BuildGateExplainReport projection when requested,
(c) top-level ordinary source-path rebase/selection.
```

The full decision set and the top-level source path remain different types;
only their relation is sealed. No second AST walk may re-decide explain
semantics. I0-A/B may explicitly leave explain cutover parked, but I0-C cannot
open until this full decision-set boundary is implemented and parity-tested.

Fuel is configured once on parser construction (`debug_fuel = fuel`) and is
not silently dropped by a wrapper. Metadata is taken once by the envelope;
AST-only wrappers discard it, while metadata wrappers project it. Explain is
captured only when demanded and is projected from the shared decision set.

## Compatibility delegate boundary

Interface/static/record compatibility must not be implemented as a hidden
catch around `delegate_lowering::lower_delegate_exposes`. The total coordinator
has one explicit compatibility arm. Before I0, a census/guard must establish
that each compatibility cohort has no delegate syntax requiring a second
source-authority relation. If a cohort later needs delegate lowering, it gets
an explicit cohort-scoped compatibility owner and removal condition; it never
shares the ordinary source-seal arm through fallback.

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
each cohort. Positive parity must compare AST shape, metadata, fuel behavior,
and diagnostics; it must not compare source-seal internals for compatibility
variants. The matrix must include direct and wrapper APIs, `NyashParser::parse`,
selected top-level gates, ordinary/interface/static/record/mixed programs,
metadata consumers, explain consumers, exact/zero/exhausted fuel, malformed
and foreign source relations, and sync-Box parity.

## Ordered implementation task map (parked until this design closes)

```text
PARSER-PUBLIC-AST-POSTPASS-S0
  private total envelope, typed cohort admission, one coordinator, caller census
  and ordinary/compatibility/no-fallback negative matrix; no public switch yet

PARSER-PUBLIC-AST-POSTPASS-I0-A
  switch the string/build-config wrapper family through the coordinator;
  preserve fuel and AST/diagnostic parity; update reference and README same commit

PARSER-PUBLIC-AST-POSTPASS-I0-B
  switch NyashParser::parse and metadata projection through the same owner;
  preserve parser-state and metadata behavior; no re-tokenization

PARSER-PUBLIC-AST-POSTPASS-I0-C
  add full BuildGate decision-set projection and explain parity;
  switch explain route only after the shared decision set is green

PARSER-PUBLIC-AST-POSTPASS-FINAL
  census direct old postpass callers, retire the old whole-root delegate edge,
  and prove no retry/fallback remains
```

Each implementation row must update the parser README, the affected language
reference, focused positive/negative parity tests, the task map, CURRENT_STATE,
and the consolidated guard in the same commit. The current D0 row itself adds
no code, fixture, production switch, or guessed `Verified*`/`Prepared*` type.

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
