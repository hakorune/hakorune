---
Status: accepted design correction; ordinary-only parser module-row I0 is bounded, static Main is a separate design stop
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-D0
ParentDecision: NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0
Candidate: B-prime-narrowed
ProductionCaller: 0
ProductionEdit: parser-only source product; ingress and production switch forbidden
---

# NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-D0

## Six-line brief

```text
Decision:
  close the smallest parser-owned ordinary module source-row authority before ingress;
  do not mix static Main entry semantics into this row product.
Source authority + canonical issuer:
  one parser invocation, existing ParserBoxSourceSealV1,
  PreparedCallableSourceV1, and exact ordinary-box source relations; one private
  ParserNormalModuleSourceAuthorityIssuerV1 at the parser product boundary.
Non-authority:
  AST-only request roots, names/ordinals/pointers, NormalSourcePlanClassifier,
  static Main entry selection, Builder catalog/expansion, Recipe/Join, MIR.
Fail-fast boundary:
  after total parser postpass and before NormalCompileRequest/Builder open;
  foreign, missing, duplicate, or contradictory rows terminate with no effect.
Smallest next slice:
  one ordinary top-level Box with one direct instance method, same parser
  invocation, no static/interface/record/build-gate/import rows.
Non-claims:
  no static-box parent source, Main.main admission, normal ingress switch,
  imports-bearing modules, body plan, resolver semantics, physical lowering,
  fallback, or GeneralProgram aggregate.
```

## Authority boundary

The parser already owns the relevant source evidence in separate products:

```text
ParserNormalProgramSourceAuthorityDispositionV1
  -> parser invocation witness + ProgramBody rows
ParserBoxSourceSealV1
  -> Box declaration site + member/constructor source relations
PreparedCallableSourceV1
  -> parser-issued top-level/member callable anchors and placements
```

The new product is an aggregate of these existing parser receipts. It must not
scan the AST a second time, derive identity from a name or ordinal, or promote
the resolver handoff into a semantic target authority. The aggregate may carry
source syntax such as diagnostic names and arity, but identity is the parser
invocation plus the opaque source relation/anchor.

The sole issuer is design-only:

```text
ParserNormalModuleSourceAuthorityIssuerV1::issue_once(
    CompletedParserPostpassV1,
    ParserCallableParameterSourceDispositionV1,
    ParserNormalProgramSourceAuthorityDispositionV1,
)
```

It is called once from the existing parser product construction boundary. It
co-seals the existing ordinary Box/callable products. It does not select a
runtime symbol, allocate a ValueId, resolve a method target, or issue a Recipe.

`static box Main` is deliberately outside this issuer. The current parser
postpass treats static and ordinary Box declarations as different cohorts, and
their mixture as `MixedProgram` compatibility. Static parent source authority
and `Main.main` admission therefore belong to a separate design card.

## Bounded cohort

The first slice is deliberately narrower than the full normal module corpus:

```text
Program root
exactly one ordinary top-level Box
exactly one direct instance method in that Box
method is parser-direct and belongs to the same Box source relation
no static/interface/record Box, BuildGate, Using, Import, nested Program,
generated-only row, or duplicate
```

The method body is only a source declaration row in this D0. Its body grammar
is not admitted. Fields, constructors, static helpers, multiple user Boxes,
static `Main.main`, imports, and top-level functions are later slices.

## Finite disposition table

| State | Sole owner | Pre-effect behavior | Fallback |
| --- | --- | --- | --- |
| `Ready` | parser module-source issuer | move one complete authority | none |
| `SourceAuthorityUnavailable` | parser postpass/source handoff | terminal, no request | no AST retry |
| `Incomplete` | parser coverage validator | terminal, no request | no empty/default row |
| `IntegrityInvalid` | parser invocation/row co-seal | terminal, no request | no name/ordinal repair |
| `Outside` | parser cohort classifier | explicit out-of-slice terminal | no normal fallback |
| `CompatibilityOutOfScope` | total postpass compatibility arm | compatibility owner only | no normal reclassification |

`Ready` requires all of the following under one parser invocation:

```text
ProgramBody coverage is total
ordinary Box count = 1
direct instance method count = 1
method-to-Box source relation is exact and unique
all Box/callable/parser brands agree
all source relations are unique and final-placement coverage is exact
```

`Outside` is a complete observation outside this bounded cohort. Missing or
contradictory rows are never `Outside`; they are `Incomplete` or
`IntegrityInvalid`.

## Move and loan contract

```text
CompletedParserPostpassV1
  + existing parser source products
  -> ParserNormalModuleSourceAuthorityDispositionV1
  -> one normal-ingress handoff in a later D0
  -> NormalCompileRequestV1 only by move
  -> Builder session only after source classification
```

The parser authority is non-`Clone`, has no public constructor, and exposes no
independent `into_parts` that permits a caller to pair Box rows with callable
rows from another invocation. Any AST view is HRTB-scoped to a parser-owned
validation callback and cannot enter the product.

The no-import condition is a named cohort fact, not an empty imports map. An
imports-bearing source must remain outside this slice until its normalized
source/import/config snapshot has a separate exact owner and can be co-sealed
without rereading the source.

## NoSafeSlice conditions

Return to `NoSafeSlice` and do not implement if any condition holds:

```text
ParserBoxSourceSealV1 lacks exact relation coverage for the ordinary cohort
PreparedCallableSourceV1 cannot identify the direct instance method through its
  parser-issued source relation without AST name/ordinal reconstruction
static Box or Main.main admission is required for the ordinary row product
two parser issuers or a second AST scan are required
the aggregate can be Clone'd, independently constructed, or partially moved
foreign parser brands cannot be rejected before the handoff
imports/config must be silently defaulted or reread
the normal request or Builder session must be opened to discover a row
```

## D0 acceptance packet

No implementation is authorized until the design packet proves:

```text
issuer definition/call site                     = 1
ParserBoxSourceSealV1 reuse                      = exact and same invocation
PreparedCallableSourceV1 reuse                   = exact and same invocation
static/Main entry observation                    = 0 in this D0
AST scan below parser boundary                   = 0
NormalCompileRequest construction                = 0 in this D0
Builder effect during row issuance               = 0
fallback/retry/reselection                       = 0
source body plan / Recipe / Join / MIR authority = 0
```

The next task after this D0 is a parser-only implementation slice with focused
positive/negative evidence and a reusable structural guard. It may issue only
this parser source disposition; it may not connect the normal ingress or add a
production caller in the same slice.

## Explicit non-claims

```text
NormalGeneralProgramModuleSourceIssuerV1 implementation
NormalCompileRequest transport change
normal/default production switch
static Box parent source and `Main.main` admission
imports-bearing or `Main(args)` admission
resolver semantic owner forest
function/body Facts or Recipe
Builder/module catalog replacement
physical entry selection or publication
legacy retirement, fallback changes, backend, performance
```

## Audit resolution and task order

The original cohort was rejected by both local inspection and the read-only
worker audit. In the current parser:

```text
ordinary `box` method       -> is_static = false
`static Main.main/0`        -> method inside `static box Main`
static + ordinary program   -> MixedProgram compatibility
static Box source seal      -> not issued by the current ordinary seal path
```

Therefore the design is split rather than repaired with an AST name lookup.
The accepted implementation task is:

```text
NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-I0
  parser-only ordinary Box source-row aggregate
  one issuer / one parser invocation / no downstream effect
```

The parked follow-up is:

```text
NORMAL-GENERAL-PROGRAM-PARSER-STATIC-BOX-PARENT-SOURCE-D0
  static Box header/member source authority and postpass cohort policy
```

The follow-up must not be implemented as part of this I0. In particular,
`static Main.main` must not be inferred from an ordinary Box name, callable
ordinal, AST scan, or compatibility row.

## I0 implementation receipt

`NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-I0` is implementation-complete as
a parser-only slice. `ParserNormalModuleSourceAuthorityIssuerV1::issue_once`
is the single call from the existing parser source-authority issuer. It
co-seals the existing ordinary `ParserBoxSourceSealV1`, callable catalog row,
direct callable path, and one parser invocation into a non-`Clone` disposition.
The disposition is carried through the existing parser authority and transform
rebuild without a second AST scan or a parallel downstream field.

Evidence:

```text
module_rows focused tests                         3 passed
script_source_authority focused tests             3 passed
source_seal_finalizer focused tests               7 passed
cargo check                                       passed
module-row structural guard                       passed
Box syntax guard                                  passed
current-state pointer guard                       passed
rustfmt changed-file check                        passed
git diff --check                                  passed
```

The broader `cargo test callable_parameter_source --lib` run exposed one
pre-existing baseline failure in the unchanged
`unchanged_parser_scan_loop_box_has_four_methods_and_fifteen_rows` test: the
parser Box declaration-syntax I0 now preserves an explicit `i64` spelling,
while that old assertion still expects `None`. It is not caused by this
module-row slice and remains classified as baseline debt; the new focused
authority tests are green.

The production caller count remains zero by design. No
`NormalCompileRequest`, resolver semantic product, Builder effect, Recipe,
Join, fallback, or production switch is part of this receipt. The next design
stop remains `NORMAL-GENERAL-PROGRAM-PARSER-STATIC-BOX-PARENT-SOURCE-D0`.
