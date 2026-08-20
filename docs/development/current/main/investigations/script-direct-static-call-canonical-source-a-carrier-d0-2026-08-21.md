---
Status: Accepted design stop — implementation remains closed
Date: 2026-08-21
Decision: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-CARRIER-D0
Parent: docs/development/current/main/investigations/script-direct-static-call-canonical-source-a-input-d0-2026-08-21.md
ProductionCaller: none; design only
ReplacementCell: one parser/frontdoor-to-compiler carrier before Source-only A
Classification: BoxShape (transport-only design; no accepted source shape)
NextCard: SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-CARRIER-I0
---

# SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-CARRIER-D0

## Six-line brief

Decision: Define one move-only carrier that transports the already-issued
parser-backed Script input from the normal-file front door to the future
canonical Source-only A boundary. The carrier is transport-only; it does not
issue semantic A facts or alter the existing Script route.

Source authority + canonical issuer: `CanonicalParserSourceHandoffV1` remains
the sole parser/source authority and co-seals parser rows with one lineage,
profile, and read/parse receipt. A compiler-side carrier sibling owns only the
move and lifetime; `CanonicalScriptDirectStaticSourceOnlyIssuerV1` is the sole
future A semantic issuer.

Non-authority: separate `script_input` fields, `SealedNormalScriptSourceV1`
alone, `CanonicalCoreSourcePlanCompileRequestV1`'s current plan/receipt pair,
AST/pointer/name/ordinal/digest joins, Builder/`comp_ctx`, `RawScriptBodyRecipeV1`,
and an explicit drop cannot issue or consume A meaning.

Fail-fast boundary: at the current named discard in
`into_canonical_core_compile_request()`, validate one source-family/identity
co-seal and move the carrier exactly once before `prepare_script_recipe()`,
`OpenScriptPhysicalEntryV1`, Builder install, or child effects. Any missing,
foreign, partial, or contradictory carrier stops before the old Script recipe.

Smallest next slice: `SCRIPT-DIRECT-STATIC-CALL-CANONICAL-SOURCE-A-CARRIER-I0`
may implement only this transport and its exhaustive state handling. A issuer,
Recipe/Join, physical Call/publication, production switch, and raw retirement
remain closed.

Non-claims: no new Script syntax, source admission expansion, resolver forest,
target/result/proof/terminal fact, A/C/B semantic package, canonical consumer,
fallback, retry, ABI/backend, or performance claim.

## Why a carrier is a separate boundary

The current route is:

```text
CanonicalParserSourceHandoffV1
  -> PreparedNormalFileSourcePlanRequestV1
  -> ClassifiedNormalFileSourcePlanV1
  -> script_input.discard_before_a_consumer()
  -> CanonicalCoreSourcePlanCompileRequestV1 { plan, admission, receipt }
  -> compile_script()
  -> prepare_script_recipe()
```

The parser rows are already AST-free and correctly one-shot, but the current
compiler request does not carry them. Adding A at `compile_script()` without
first closing this seam would make A depend on a missing input or force a
second parser scan. The carrier is therefore a behavior-preserving transport
slice, not a semantic implementation shortcut.

The compiler-side type must not import a runner-owned receipt type as its
authority. Put the transport model in a thin compiler/source-plan sibling (or
move the neutral row carrier to the parser/source-plan boundary), and let the
front door construct it through one private co-seal. Keep
`canonical_core_dispatch.rs` and the large lifecycle owners at forwarding-only
growth; the new owner must stay below the 760/800-line limits.

## Exhaustive upstream and transport state table

The carrier must preserve every upstream state and must not reinterpret it as
an A disposition. `HandoffConsumed` means a real A consumer took the rows;
until then, a no-consumer rejection is `DiscardedBeforeA`.

| state | phase | issuer / authority | pre-effect behavior | terminal / continuation | fallback policy |
|---|---|---|---|---|---|
| `CanonicalScriptCohortAdmitted` | parser admission | parser cohort issuer plus complete parameter source | admit one row issuance | continue to row issuer | never infer from a bool |
| `CohortUnresolved` | parser admission | parser cohort issuer cannot prove pure Script | no row/carrier effect | typed stop or compatibility owner | never empty-success or A |
| `AdmissionMissing` | parser admission | selected build-gate/unsupported source product | no row/carrier effect | explicit non-canonical terminal | never `HandoffReady` |
| `CompatibilitySource` | upstream | parser/source admission | preserve reason and lineage | compatibility owner or stop | never canonical A |
| `Deferred` | upstream | resolver/source admission | preserve deferred reason | deferred owner or stop | never `NonCandidate` |
| `SourceAuthorityUnavailable` | upstream | parser identity/profile/receipt preflight | stop before carrier | `NoSafeSlice` | no default/rescan |
| `HandoffReady` | frontdoor transport | parser rows + one source-bound profile/receipt co-seal | move exactly once | carrier input or explicit discard | no `script_input: _` silent drop |
| `DiscardedBeforeA` | transport terminal | named rejection/no-A owner | no compiler candidate publication | terminal discard | never report `HandoffConsumed` |
| `HandoffConsumed` | A transport terminal | the named A consumer takes the carrier | no replay or second read | A observation starts | no use without a real consumer |
| `DispositionTransported` | future C-to-B phase | C/B typed transport owner | no source reinterpretation | detached consumer terminal | not an A or parser state |

Each row has one owner, one pre-effect behavior, one continuation, and one
fallback policy. `NoSafeSlice` remains a development stop and is not a source
state. `NotApplicable` is handled by the outer canonical family classifier
before this carrier and cannot be fabricated from a missing carrier.

## Carrier payload and identity contract

The move-only carrier may contain only the already-issued, AST-free source
input:

```text
parser-issued ProgramBody/declaration/Brand/config rows
one source-bound parser lineage/profile/receipt witness
source digest and UTF-8/read/parse facts only as that witness exposes them
canonical Script-family admission witness
```

It must not contain:

```text
AST or AST pointer
Builder/comp_ctx state
FunctionOwnerIdV1 or resolver forest
target/result/Recipe/Join/proof/terminal rows
ValueId, MirType, MIR block, physical ID, or Recipe key
```

The carrier is not allowed to pair independent primitive fields after move.
The parser/front-door co-seal must reject a foreign parser brand, foreign
lineage/profile, source digest/UTF-8/count drift, incomplete import snapshot,
or mismatched Script cohort before `HandoffReady` is issued. A filename,
statement ordinal, pointer, name, path, or digest-only equality is never a
pairing key.

## Lifetime and owner boundary

```text
parser/front door
  owns source parse and issues HandoffReady once
  -> carrier move
compiler/source-plan boundary
  owns transport only and retains it through Script plan/request
  -> future A issuer consumes HandoffConsumed once
Source-only A
  issues resolver/target/result/proof/terminal facts later
```

The carrier must travel only with the Script family. Main, callable, AST-only,
compatibility, deferred, and source-free plans preserve their existing typed
owners and do not receive an empty Script carrier. A missing carrier for a
Script plan is `SourceAuthorityUnavailable`/`ObservationIncomplete` according
to whether observation can begin, never `NonCandidate` or old raw success.

The existing explicit `discard_before_a_consumer()` is a temporary no-A
terminal. I0 must replace it at the selected Script boundary with either the
carrier move or a named `DiscardedBeforeA` terminal. After a real A consumer
exists, that discard API must be deleted or guarded so `HandoffReady` cannot be
silently dropped.

## Acceptance matrix

Positive:

- canonical no-import pure Script with matching parser brand/lineage/profile/
  receipt moves one carrier from front door to compiler request;
- one carrier is visible only on Script plan, while Main/Callable keep their
  existing ownership and no empty Script field;
- carrier move is linear: a second move/consume, clone, replay, or reparse is
  rejected;
- carrier payload remains AST-free and contains no Builder, resolver, target,
  Recipe, Join, physical, or publication fact;
- future A receives the same source witness without reconstructing identity.

Negative:

- `CompatibilitySource`, `Deferred`, `AdmissionMissing`, and
  `CohortUnresolved` never become `HandoffReady`;
- foreign/missing parser brand, profile, digest, UTF-8 length, read/parse
  counts, or import/config completeness stops before the old Script recipe;
- Script plan with missing carrier is a typed stop, not `NonCandidate`, raw
  fallback, or empty success;
- Main/Callable/AST-only/source-free plans cannot borrow a Script carrier;
- rejected front-door and canonical-request paths name `DiscardedBeforeA`
  instead of silently destructuring `script_input: _`;
- no pointer/name/ordinal/digest-only pairing or AST rescan is accepted;
- an A implementation cannot be reached directly from `compile_script()` while
  the carrier is absent.

## NoSafeSlice conditions

Remain at this D0 if any condition holds:

1. the compiler type must import a runner-owned receipt as its source authority;
2. parser rows and lineage/profile/receipt can be moved separately and paired
   later;
3. the Script carrier is stored as `Option`/empty/default and missing means
   ordinary raw Script;
4. `HandoffConsumed` can be emitted without a named A consumer;
5. a carrier payload requires AST, Builder state, resolver/target/Recipe/Join,
   physical IDs, or a second semantic issuer;
6. Main/Callable/Compatibility/Deferred/source-free plans are forced through
   an empty Script carrier;
7. `compile_script()` must rescan/reparse or pair by pointer/name/ordinal/digest;
8. the transport requires semantic growth in `canonical_core_dispatch.rs`,
   the 760-line frontdoor, or any source at the 760/800 limit.

Until these are closed, A issuer implementation and canonical physical
consumer remain design-only.

