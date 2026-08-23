---
Status: Parser-only I0 implemented; closeout evidence recorded
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-STATIC-BOX-PARENT-SOURCE-D0
Exception: independent parser source-authority boundary; ordinary module-row card is closed
ParentCurrentCard: docs/development/current/main/investigations/normal-module-parser-source-rows-d0-2026-08-23.md
ProductionCaller: 0
ProductionEdit: parser-only static parent source transport/seal I0; no downstream consumer
CeremonyTier: T2 — new parser source authority and identity boundary
---

# NORMAL-GENERAL-PROGRAM-PARSER-STATIC-BOX-PARENT-SOURCE-D0

## Current Capsule

- **Current decision:** static Box parent source is a separate parser authority;
  it is not an extension of the ordinary `ParserBoxSourceSealV1`.
- **Current implementation status:** parser-only I0 now issues one separate
  static parent disposition and transports it as a `CompletedParserPostpassV1`
  sibling; no downstream caller was added.
- **Next ordered task:** keep static/mixed postpass policy and `Main.main`/App
  admission as separate design decisions; no downstream implementation is open.
- **Production stop line:** no `NormalCompileRequest`, `Main.main` admission,
  Builder effect, Recipe/Join, fallback, or compatibility reclassification.
- **Retirement finish line:** one static parent authority is named, all old
  static-parent source claims are classified, and no second source issuer or
  ordinary-seal bypass remains.

## Six-line brief

```text
Decision:
  use a separate ParserStaticBoxSourceSealV1 for one bounded static Box parent;
  keep ordinary ParserBoxSourceSealV1 unchanged.
Source authority + canonical issuer:
  parse_static_box header + parser invocation brand/path + exact member cursor
  coverage + existing direct static callable source rows; one private
  ParserStaticBoxParentSourceAuthorityIssuerV1::issue_once at the postpass
  finalizer boundary.
Non-authority:
  Main name/entry selection, AST rescans, method names/ordinals/pointers,
  runtime registries, NormalSourcePlan, MIR, Builder, Recipe/Join, compatibility
  fallback, and any ordinary seal extension.
Fail-fast boundary:
  after static parser source finalization and before postpass normal handoff;
  missing/foreign/duplicate/contradictory parent or member evidence terminates
  without a request or physical effect.
Smallest next slice:
  one top-level static Box, one direct static method, same parser invocation,
  bundled header/member rows, no fields/init/generated rows; transport only.
Non-claims:
  Main.main/App selection, mixed programs, multiple static Boxes, inheritance,
  imports, resolver semantics, Recipe/Join, MIR, publication, fallback, and
  production switch.
```

## Decision and authority boundary

The current parser has two different facts that must not be merged:

```text
static Box AST + direct static callable catalog rows
    !=
ordinary ParserBoxSourceSealV1
```

`parse_static_box` already has the parser-owned header context, one
`ParserInvocationBrandV1`, one `SourceBoxDeclarationSiteV1`, and a
`ParserBoxMemberSourceCursorV1`. Direct static methods already publish
`SourceBoxMethodSiteV1::Direct` and `StaticBoxMethod` callable rows into the
existing parser catalog. The missing authority is the static parent relation
and its total member coverage.

The selected design is therefore a new, separate product:

```text
ParserStaticBoxParentSourceAuthorityIssuerV1
  -> ParserStaticBoxSourceSealV1
  -> static-parent disposition
```

The issuer co-seals only parser-owned evidence from one invocation:

```text
static header syntax (kind/static, diagnostic name, sync state)
parser invocation witness/brand
exact Box declaration path
ordered member-coverage witness from the source cursor
direct static method source relation, when admitted by this cohort
```

The parent seal is a source-preservation/coverage product. It does not decide
that a method is an entry, a target, a candidate, or a physical operation.
The ordinary seal remains ordinary-only because its finalizer, delegate policy,
postpass `SourceSealedOrdinary` row, and relation vocabulary are all bounded
to that cohort.

Extending `ParserBoxSourceSealV1` is rejected: it would make the ordinary
finalizer and static compatibility arm share a false authority, while the
static parser currently has no parent seal issuance path.

## D0 closure: exact I0 contract

The design is now closed at the following two-stage parser boundary:

```text
parse_static_box
  -> one ParserStaticBoxSourceTransactionV1
  -> one opaque PreparedParserStaticBoxParentSourceV1
  -> existing parser callable-source/postpass move
  -> finish_total_s0
  -> ParserStaticBoxParentSourceAuthorityIssuerV1::issue_once
  -> CompletedParserPostpassV1 static sibling disposition
  -> ParsedProgramWithCallableParameterSourceV1::new moves that sibling
```

The transaction emits one bundled row per source member. A row contains its
parser-branded `SourceBoxMemberSiteV1`, a closed member-kind witness, and the
direct static method relation only when that member is a direct method. The
final coverage witness owns the exact contiguous member count and the Box
path; it is not a bare ordinal list. Existing static callable catalog rows are
co-sealed by exact brand/path/member-site equality, never by a name/ordinal
join.

The first I0 admits only:

```text
one top-level static Box
one direct static method
no field/init/static-init/constructor/generated member
no build-gate path, ordinary sibling, interface, record, or import
```

Other observed member kinds receive an explicit `Outside`/typed source
disposition; they are not omitted from a supposedly total parent product.
`Main` has no special source identity here. `Main.main` entry/App selection is
not consumed and remains a separate `Outside` boundary at its later consumer.

The static prepared payload must survive the existing postpass prune move by
the same parser brand/path. It is carried as a sibling of ordinary prepared
seals, never inserted into `prepared_source_seals` and never converted into
`SourceSealedOrdinary`. The final static issuer runs once in
`OpenParserPostpassProductV1::finish_total_s0`;
`ParsedProgramWithCallableParameterSourceV1::new` only transports the completed
sibling and does not reissue it. No AST rescan is permitted.

## Bounded cohort

The first source-only cohort is intentionally narrow:

```text
one top-level static Box in one parser invocation
one direct static method with an existing callable source row
no ordinary Box or mixed program
no fields, init/static-init, constructors, generated members, inheritance,
  interface, build gate, Using, or Import rows
no entry selection and no semantic method-body admission
```

The positive fixture should use a non-entry name such as `Utility`. `Main` is
not an identity key and is never selected by this issuer. Any `Main.main`
entry request remains `Outside` at the later entry/consumer boundary; this D0
does not use the name `Main` to manufacture an entry fact or a fallback.

The member cursor may be used only through a parser-issued total coverage
witness. A final ordinal alone is not a method identity and cannot be joined
with the callable catalog by position. The method relation must carry the same
parser brand and exact Box path.

## Finite disposition table

| State | Sole owner | Pre-effect behavior | Allowed terminal/fallback |
| --- | --- | --- | --- |
| `Ready` | static parent source issuer | move one complete parent seal | source handoff only; no fallback |
| `Outside` | static cohort classifier | complete but outside this bounded cohort | typed terminal; no normal reclassification |
| `SourceAuthorityUnavailable` | parser source boundary | no parent source product is available | typed terminal; no AST retry |
| `Incomplete` | static coverage validator | required header/member/method evidence is missing | typed terminal; no empty/default row |
| `IntegrityInvalid` | invocation/relation co-seal | foreign, duplicate, stale, or contradictory evidence | typed terminal; no name/ordinal repair |
| `CompatibilityOutOfScope` | existing compatibility owner | static/mixed input remains on its existing compatibility lane | compatibility only; no normal fallback |

`Ready` requires one parser invocation, one exact Box path, one complete static
header witness, contiguous total member coverage, and one exact direct static
method relation for this cohort. A complete unsupported shape is `Outside`; a
missing row is never silently treated as `Outside`.

## Fail-fast and non-authority rules

The boundary is:

```text
static parser source finalization
  -> static parent issuer
  -> typed disposition
  -> stop before normal postpass handoff / NormalCompileRequest / Builder
```

The following cannot issue or repair this seal:

```text
AST scan after parsing
name == "Main"
method name or inventory ordinal
Span, AST pointer, digest, or raw usize
ParserCallableParameterSourceCatalogV1 alone
NormalSourcePlanClassifier / Main source planner
BoxCallableRegistry or runtime provider registry
normal/static compatibility arm alone
MIR paths, ValueId, BasicBlockId, Builder products
```

The callable catalog is an input relation, not the parent authority. The
source cursor is an observation primitive, not a seal by itself. The aggregate
may co-seal existing parser receipts but may not invent method semantics,
target membership, result type, or entry meaning.

## Acceptance evidence for parser-only I0

The parser-only implementation is accepted when the following remain fixed and
observable:

```text
static parent issuer definition/call site                 = 1
static parent seal constructor outside issuer             = 0
ordinary ParserBoxSourceSealV1 extension                   = 0
same parser brand/path for every retained relation         = 1
positive non-entry static Box source fixture               = 1
missing/foreign/duplicate/contradictory negative cases     = typed and pre-effect
Main.main semantic admission in this slice                = 0
NormalCompileRequest / Builder / Recipe / Join effects      = 0
AST/name/ordinal/pointer re-pairing                         = 0
fallback/retry/reselection                                  = 0
source files                                                < 760 lines
```

The implementation has one parser-focused positive/negative gate and one
reusable structural guard. It proves the product moves through parser source
transport without adding a downstream field or a production caller.

## NoSafeSlice conditions

Return to `NoSafeSlice` and do not implement if any of these is true:

```text
static parent can only be represented by extending ordinary ParserBoxSourceSealV1
the parser cannot issue exact header/member coverage without an AST rescan
member coverage requires joining independent arrays by ordinal or name
static method rows cannot be tied to the same brand and Box path
fields/init/generated members must be silently defaulted or omitted from a
  supposedly total parent product
mixed ordinary/static postpass policy must change in this slice
Main.main entry semantics are required to prove the parent source row
no single parser issuer/call site can be named
the product needs a NormalCompileRequest, Builder, Recipe, Join, MIR, or fallback
the source schema cannot stay below the 760-line split trigger
```

## Ordered task queue

```text
D0  this card: freeze static parent authority, cohort, states, and hard stops
NORMAL-GENERAL-PROGRAM-PARSER-STATIC-BOX-PARENT-SOURCE-I0:
    complete: issue one parser-owned static parent seal only
D0  separate: decide static/mixed postpass transport policy
D0  separate: decide Main.main/App entry admission from the parent source
later: normal ingress -> Facts -> Recipe -> Verify -> Lower, only after those
       source and consumer authorities are independently named
```

The D0 is accepted and the parser-only I0 is implemented. The change adds the
new parser source transaction, sibling transport field, final static issuer,
focused parser tests, and one reusable guard. It does not connect normal
ingress, add a production caller, or open Main/App/Builder/Recipe/Join/MIR/
fallback.

## I0 implementation receipt

```text
issuer definition/call site              = 1 / 1
ordinary ParserBoxSourceSealV1 extension = 0
static source sibling transport          = 1
AST/name/reparse/downstream authority    = 0
production caller                        = 0
focused static-parent tests               = 5 passed
postpass envelope tests                   = 7 passed
cargo check                               = passed
reusable static-parent guard              = passed
current-state pointer guard               = passed
git diff --check                          = passed
source/fixture files                      < 760 lines
```

The broader callable-parameter suite has one pre-existing baseline failure:
`unchanged_parser_scan_loop_box_has_four_methods_and_fifteen_rows` expects an
untyped `pos`, while the checked-in fixture declares `pos: i64`. It is outside
this static-parent diff and remains classified as baseline debt; this I0 does
not alter that fixture or test.

## Worker/census receipt

The read-only downstream audit confirmed that normal callers still route static
and mixed programs through compatibility arms, while direct static callable rows
already exist in the parser catalog. It also confirmed that `Main.main`, App
selection, raw static-main expansion, and runtime registries are separate
authorities. This supports the separate static parent issuer and the explicit
non-claims above; it does not authorize implementation.
