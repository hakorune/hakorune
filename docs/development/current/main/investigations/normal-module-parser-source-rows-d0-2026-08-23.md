---
Status: closed design stop; NoSafeSlice found, Box declaration syntax prerequisite selected
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-D0
ParentDecision: NORMAL-GENERAL-PROGRAM-MODULE-SOURCE0-D0
Candidate: B-prime
ProductionCaller: 0
ProductionEdit: forbidden during D0
---

# NORMAL-GENERAL-PROGRAM-PARSER-MODULE-ROWS-D0

## Six-line brief

```text
Decision:
  close the smallest parser-owned module source-row authority before ingress.
Source authority + canonical issuer:
  one parser invocation, existing ParserBoxSourceSealV1,
  PreparedCallableSourceV1, and exact entry observation; one private
  ParserNormalModuleSourceAuthorityIssuerV1 at the parser product boundary.
Non-authority:
  AST-only request roots, names/ordinals/pointers, NormalSourcePlanClassifier,
  Builder catalog/expansion, Recipe/Join, runtime entry selection, MIR.
Fail-fast boundary:
  after total parser postpass and before NormalCompileRequest/Builder open;
  foreign, missing, duplicate, or contradictory rows terminate with no effect.
Smallest next slice:
  one ordinary Program with one static Main.main/0 and one plain non-Main Box
  with one direct method, same parser invocation, no import/build-gate rows.
Non-claims:
  no normal ingress switch, imports-bearing modules, body plan, resolver
  semantics, physical lowering, fallback, or GeneralProgram aggregate.
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
co-seals the existing Box/callable products and the source-level entry
observation. It does not select a runtime symbol, allocate a ValueId, resolve
a method target, or issue a Recipe.

The entry observation means only:

```text
the parser-issued Main declaration relation contains static main/0
```

Runtime `Main.main -> root main` expansion remains a later admission/physical
owner. This prevents parser syntax carriage from becoming Builder policy.

## Bounded cohort

The first slice is deliberately narrower than the full normal module corpus:

```text
Program root
exactly one ordinary (non-static/non-interface/non-record) Main Box
exactly one direct static Main.main/0 declaration
exactly one ordinary non-Main Box
exactly one direct instance method in that Box
no BuildGate, Using, Import, nested Program, generated-only row, or duplicate
```

The non-Main method body is only a source declaration row in this D0. Its body
grammar is not admitted. Fields, constructors, static helpers, multiple user
Boxes, imports, `Main.main(args)`, and top-level functions are later slices.

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
Main Box count = 1
Main.main declaration count = 1 and static arity = 0
non-Main ordinary Box count = 1
non-Main direct method count = 1
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
ParserBoxSourceSealV1 lacks exact relation coverage for the cohort
PreparedCallableSourceV1 cannot identify Main.main and the user method without
  AST name/ordinal reconstruction
the entry observation would choose a runtime/physical route
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
entry observation is syntax-only                 = yes
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
imports-bearing or Main(args) admission
resolver semantic owner forest
function/body Facts or Recipe
Builder/module catalog replacement
physical entry selection or publication
legacy retirement, fallback changes, backend, performance
```
