---
Status: Design accepted — policy frozen; no implementation authorized
Date: 2026-08-23
Decision: NORMAL-GENERAL-PROGRAM-PARSER-STATIC-MIXED-POSTPASS-POLICY-D0
ParentCurrentCard: docs/development/current/main/investigations/normal-static-box-parent-source-d0-2026-08-23.md
ProductionCaller: 0
ProductionEdit: none; policy/census only
CeremonyTier: T2 — postpass cohort policy boundary
---

# NORMAL-GENERAL-PROGRAM-PARSER-STATIC-MIXED-POSTPASS-POLICY-D0

## Current capsule

- **Current decision:** keep the bounded static-parent source disposition
  separate from the ordinary `ParserBoxSourceSealV1`; mixed programs do not
  receive a partial static `Ready` row.
- **Current implementation status:** parser-only static-parent I0 is complete
  and transported as a `CompletedParserPostpassV1` sibling. This card only
  freezes the next postpass policy; it adds no code or production caller.
- **Next ordered task:** open the separate `Main.main`/App admission D0. This
  policy remains parser/postpass-only and does not authorize a consumer.
- **Production stop line:** no `NormalCompileRequest`, normal source-plan
  promotion, Builder effect, Recipe/Join, MIR, fallback, or compatibility
  reclassification is open.
- **Retirement finish line:** one static source issuer remains, mixed partial
  promotion is impossible by policy, and the existing compatibility route is
  not mistaken for a canonical static consumer.

## Six-line brief

```text
Decision:
  preserve separate static-parent source disposition; ordinary remains
  SourceSealedOrdinary; mixed is an explicit typed compatibility/Outside
  boundary and never a partial static Ready.
Source authority + canonical issuer:
  the existing parser-owned static header/member/callable relation and the
  sole ParserStaticBoxParentSourceAuthorityIssuerV1::issue_once; this D0 adds
  no second semantic issuer, only a postpass policy decision around it.
Non-authority:
  classify_program alone, ordinary ParserBoxSourceSealV1, static callable rows
  alone, Main/name matching, AST re-scan, raw compatibility, NormalSourcePlan,
  runtime registry, Builder, Recipe/Join, MIR, and fallback.
Fail-fast boundary:
  after one same-invocation cohort classification and static disposition, and
  before any static/ordinary promotion to NormalCompileRequest or physical work.
Smallest next slice:
  design-only finite policy/census for ordinary, bounded static, mixed,
  interface/record/build-gate, no-box, and non-program cohorts; no code.
Non-claims:
  Main.main/App admission, static semantic entry selection, mixed consumption,
  normal ingress, resolver, Recipe/Join, MIR, publication, fallback, and switch.
```

## Decision and authority boundary

The parser now has two independent source products:

```text
ordinary top-level Box
  -> ParserBoxSourceSealV1

bounded static Box parent
  -> ParserStaticBoxParentSourceDispositionV1
```

They must not be merged merely because both contain Box coordinates. The
ordinary seal's finalizer and delegate policy are bounded to the ordinary
cohort. The static parent issuer owns the static header, same parser brand and
Box path, total member coverage, and the exact direct-static callable relation
for its narrow cohort. It does not own entry selection or normal lowering.

The postpass coordinator is a routing owner, not a new source authority. It
may transport the static disposition as a sibling and choose an explicit
compatibility/Outside terminal for unsupported cohorts. It must not reconstruct
the static parent from `ASTNode`, method names, inventory ordinals, or the
ordinary seal.

## Required finite policy

| Program cohort | Ordinary source row | Static parent disposition | Policy result | Downstream effect |
| --- | --- | --- | --- | --- |
| one ordinary top-level Box | `SourceSealedOrdinary` | unavailable-for-ordinary | ordinary source lane only | none in this D0 |
| one bounded static Box, one direct static method | not ordinary | `Ready` | static sibling transport only | zero |
| static Box with unsupported member kind | not ordinary | `Outside(UnsupportedMemberKind)` | typed static terminal | zero |
| static Box with multiple/invalid direct methods | not ordinary | `Outside`/`Incomplete`/`IntegrityInvalid` | typed static terminal | zero |
| mixed ordinary + static program | compatibility row | `Outside(ProgramCohort)` | existing typed compatibility arm (`AstOnlyCompatibility` or `InitialCompatibility`); no partial promotion | zero |
| interface/record/build-gate cohort | compatibility row | explicit non-static disposition | existing compatibility only | zero |
| no-box Script cohort | no ordinary/static row | not applicable to static parent | existing Script policy; no static promotion | zero |
| non-program input | no ordinary/static row | not applicable | existing non-program policy | zero |

The exact error vocabulary for compatibility is an existing postpass concern;
this card does not introduce a new enum or convert `NoSafeSlice` into a runtime
state. The important invariant is the edge rule:

```text
mixed source
  -/-> partial static Ready
  -/-> SourceSealedOrdinary
  -/-> name/ordinal re-pairing
  -/-> normal/legacy retry
```

## Current consumers and bypass census

The static parent source currently has no production semantic consumer. Its
known current uses are parser sibling transport, typed accessors, and focused
tests. Existing static/mixed compilation continues through the existing typed
compatibility arms (`AstOnlyCompatibility` or `InitialCompatibility`); that
route is not evidence that the static parent seal has been consumed.

The D0 census must record, by symbol and file:

```text
ParserStaticBoxParentSourceAuthorityIssuerV1 definition/call count
static disposition transport count
SourceSealedOrdinary construction count
static/mixed compatibility entry count
canonical_script_source_admission callers
Main/App entry selectors
direct static physical/Builder callers
fallback/retry/reselection edges
```

The following are explicit bypasses and must remain zero for a future static
consumer slice:

```text
static source -> ordinary seal
static source -> Main/name selector without an entry product
mixed source -> partial static rows
static source -> AST re-scan or method-ordinal join
compatibility failure -> normal or legacy retry
static disposition -> Builder/Recipe/Join/MIR without a named consumer
```

## Fail-fast boundary

The policy boundary is:

```text
parser source finalization
  -> one static issuer / typed disposition
  -> program-cohort policy
  -> typed terminal or sibling transport
  -> stop before normal promotion, request, or physical effect
```

`Ready` is transport-only in this card. It is not a semantic candidate, an
entry, a target, or a lowering permit. `Outside`, `Incomplete`, and
`IntegrityInvalid` are terminal observations, not invitations to retry through
the old route.

## Ordered task queue

### D0.1 — Freeze cohort matrix

Record the finite matrix above against the actual postpass enum arms and
compatibility constructors. Any arm that cannot be classified without a new
authority is a `NoSafeSlice`, not an implicit default.

### D0.2 — Freeze the edge census

Record the sole static issuer, sibling transport, compatibility route, current
zero production consumer, and all possible bypass/fallback edges. Separate
parser tests/accessors from production consumers.

### D0.3 — Freeze the mixed rule

Accept only the rule that mixed programs remain explicit compatibility/Outside
and never promote a static subset. Do not add a partial `Ready` aggregate or
extend the ordinary seal.

### D0.4 — Freeze acceptance evidence

The design packet must show:

```text
static-only Ready remains parser sibling transport
mixed partial Ready = 0
static -> ordinary seal = 0
AST/name/ordinal repair = 0
compatibility -> normal fallback delta = 0
Main/App admission in this card = 0
production consumer = 0
```

### D0.5 — Decision gate

If the matrix and edge census remain closed, accept this D0 as the policy
contract and create a separate `Main.main`/App admission D0. If any row needs
ordinary-seal extension, a second issuer, source re-scan, or downstream
effects, return to `NoSafeSlice` and redesign that authority separately.

## NoSafeSlice conditions

Do not implement a static/mixed consumer from this card if any of these holds:

```text
ordinary ParserBoxSourceSealV1 must be extended with static semantics
mixed static rows must be joined by name, ordinal, Span, or AST pointer
static Ready requires Main/App entry semantics
compatibility construction is mistaken for canonical static consumption
two static/mixed issuers are needed
static policy cannot terminate before NormalCompileRequest/Builder effects
canonical failure requires normal/legacy fallback or re-selection
the current postpass enum cannot represent the matrix without a guessed default
the source/policy change exceeds the 760-line design split trigger
```

## Separate next decision: Main.main/App

`Main.main` and App mode remain a later, independent authority decision. The
existing `VerifiedRawRootExpansionV1`, normal source-plan Main relation, raw
root lifecycle, and semantic-package Main-child selection are not silently
reused as a parser static-parent consumer. A future entry D0 must establish
program-wide uniqueness and same-invocation relation before it can issue a
typed `AppMainReady`/Outside/Incomplete/IntegrityInvalid product.

That future card must not be opened by this policy card's implementation. It
must first name whether the parser static parent seal plus a program-level
entry inventory can replace, wrap, or remain separate from the existing raw
root authority. Until then, Main/name matching and App selection remain
non-authorities for this card.

## Acceptance packet for this design stop

```text
Decision brief present = 1
finite cohort matrix present = 1
sole issuer and postpass coordinator named = 1
mixed partial Ready rule = explicit prohibition
ordinary seal extension = 0
new semantic product/issuer = 0
production consumer = 0
fallback/retry/reselection change = 0
Main/App implementation = 0
worker audits = 3, recommendation unanimous
```

## Read-only census receipt

The current `main` census supports this policy without opening a consumer:

```text
static issuer definition/call site
  = 1 / 1
  = static_box_source.rs:255 / source_seal/finalize.rs:175

static parent production consumers outside parser/tests
  = 0

ordinary source-seal construction for the ordinary cohort
  = source_seal/finalize.rs:152-163 -> from_source_product

static/mixed postpass route
  = source_seal/finalize.rs:164-219 -> existing typed compatibility arm
    (AstOnlyCompatibility or InitialCompatibility)

static/mixed canonical Script admission
  = canonical_script_source_admission.rs:73-79 -> CompatibilitySource

mixed partial static Ready
  = 0 by issue_once requiring StaticBox cohort and exactly one prepared parent

static-to-ordinary-seal promotion
  = 0; ordinary constructor marks the static sibling unavailable-for-ordinary

Main/App authority in this card
  = 0; existing raw-root/Main-child selectors remain separate

new fallback/retry/reselection edge
  = 0
```

The `InitialCompatibility` arm is intentionally included in the evidence:
some existing compatibility-shaped programs can enter the historical initial
callable transport after `postpass_compatibility::lower`, but that transport
does not consume or reissue the static parent seal. It is therefore not a
canonical static source consumer and must not be promoted by this D0.

## D0 decision result

**Accepted as a policy contract, not as an implementation switch.** The
static sibling, ordinary seal, and existing compatibility arms remain
separate. Mixed partial promotion is forbidden. The next design card is the
independent `Main.main`/App admission boundary; no downstream static consumer
is authorized by this acceptance.
