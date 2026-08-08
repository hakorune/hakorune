---
Status: closed implementation
Date: 2026-08-09
Decision: implement the private total postpass envelope without switching a public caller
Parent: `parser-public-ast-postpass-cutover-d0-design-task-2026-08-09.md`
---

# PARSER-PUBLIC-AST-POSTPASS-S0

## Scope

Implement the first behavior-preserving slice of the accepted public AST
postpass design. This row adds one private coordinator and one typed result;
it does not switch the broad public parser APIs yet.

```text
parse_program once
  -> existing prune/delegate owners through one explicit coordinator
  -> typed cohort admission
  -> CompletedParserPostpassV1
```

The result must distinguish source authority from compatibility transport:

```text
SourceSealedOrdinary
  = completed ordinary rich product with ParserBoxSourceSealV1

AstOnlyCompatibility
  = AST + typed cohort receipt + metadata projection
  = never a source seal or resolver input
```

The coordinator owns one invocation/session. Compatibility lowering is an
explicit compatibility arm, not a catch around the ordinary arm and not a
retry. The existing public callers remain unchanged until I0-A/B/C.

## Required implementation

- Add a private `CompletedParserPostpassV1` envelope with AST, metadata,
  optional explain slot, and typed per-Box/program coverage.
- Add typed ordinary/compatibility cohort classification from the already
  parsed/pruned AST. Do not classify by name, inventory ordinal, or a second
  source scan.
- Add an explicit compatibility postpass owner that isolates the existing
  compatibility delegate lowering. It must not issue or carry a source seal.
- Add conversion from the existing ordinary `ParsedProgramWithSourceV1` into
  the `SourceSealedOrdinary` envelope variant.
- Reuse the existing parser invocation brand, source session, metadata, and
  fuel configuration; do not re-tokenize or reset fuel.
- Keep explain demand explicit. `Capture` remains parked until the full
  BuildGate decision-set row; S0 must return a typed unsupported/not-ready
  error rather than silently producing a partial report.
- Reuse the existing rich path through a single postpass-opening helper where
  this is behavior-neutral.

## Acceptance tests

```text
ordinary Box                  -> SourceSealedOrdinary
interface/static/record Box   -> AstOnlyCompatibility
ordinary + compatibility      -> mixed compatibility coverage
no Box declarations           -> typed compatibility/no-box coverage
remaining top-level gate     -> explicit diagnostic, never fallback
source coverage mismatch     -> whole product rejected/dropped
source-sealed conversion     -> seal count/brand/coverage preserved
metadata                      -> taken once and preserved by envelope
explain Capture at S0         -> explicit parked/not-ready error
```

Tests must assert that compatibility rows cannot be projected as a seal and
that the coordinator has one postpass choice. They must not claim public API
parity, full nested BuildGate explain parity, resolver connection, or caller
zero yet.

## Non-claims and stop lines

Do not implement in this row:

```text
public parse/metadata/explain caller cutover
full PreparedBuildGateDecisionSetV1
explain-report parity
resolver target or CallableContract
Recipe/CallSlot/Builder/MIR/runtime work
rich-then-legacy fallback or retry
fake/empty ParserBoxSourceSealV1
AST/name rescan or ordinal identity reconstruction
```

If a required compatibility cohort cannot be represented without guessing,
stop with `NoSafeSlice` in the card rather than inventing a Verified/Prepared
product. Keep all touched Rust files below 760 lines; split by owner before
800 lines.

## Same-commit closeout

The implementation commit must update:

```text
src/parser/README.md
docs/reference/language/callable-contracts.md
docs/development/current/main/design/parser-postpass-source-handoff-ssot.md
the callable-contract task map
CURRENT_STATE.toml
the consolidated postpass guard
focused positive/negative tests
```

## Implementation receipt (2026-08-09)

S0 landed the private `CompletedParserPostpassV1` envelope, structural cohort
classifier, explicit compatibility delegate arm, and one
`OpenParserPostpassProductV1::finish_total_s0` coordinator. The existing rich
ordinary path now uses the shared postpass-opening helper; broad public parser
callers remain unchanged. Focused tests cover ordinary source-sealed rows,
static and mixed compatibility rows, ordinary/compatibility constructor
separation, and the parked explain-capture error. The full BuildGate decision
set and public caller cutover remain unopened for I0-C and I0-A/B.

The next row is `PARSER-PUBLIC-AST-POSTPASS-I0-A`.
