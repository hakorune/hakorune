---
Status: design_closed__2026-09-14
Task: MIR-CALL-STATIC-COMPATIBILITY-A1-STATIC-PARENT-D0
Date: 2026-09-14
Parent: mir-call-static-compatibility-i0-a0-2-2026-09-14.md
NextCard: MIR-CALL-STATIC-COMPATIBILITY-I0-A1-STATIC-PARENT
Implementation permission: false; design and taskization only
---

# A1 finite static-parent source co-seal

## Six-line brief

```text
Decision: extend the existing parser-owned static-parent authority to a finite same-brand set of static parents and direct methods inside the A0 mixed source window.
Source authority + canonical issuer: the parser postpass transaction and one ParserStaticBoxParentSourceAuthorityIssuerV1 boundary issue all static parent/member/method relations; the normal source-plan surface consumes them.
Non-authority: AST shape, names/arity, ordinals without parser sites, MIR, runtime aliases, LineSpan diagnostics, static target/result catalogs, and compatibility fallback.
Fail-fast boundary: foreign brand, unsupported member kind, stale/foreign site, duplicate parent/member/callable relation, missing parameter/source row, member-count mismatch, or orphan prepared row rejects before source-backed admission.
Smallest next slice: define the finite multi-parent/multi-method relation schema and affine reject mapping; no source-admission switch or body lowering.
Non-claims: no A0 source-window implementation, A1-2 body/call/constructor handoff, A1-3 publication proof, compatibility-edge deletion, fallback restoration, Windows proof, or R7 closure.
```

## Why this is the next bounded design row

The completed A0-2 slice transports merged-source lineage and co-seals it to
one parser invocation, but it does not issue static parent relations. The
existing parser I0 in
`src/parser/callable_parameter_source/static_box_source.rs` is intentionally
limited to a pure `StaticBox` cohort, one parent, one path segment, all direct
methods, and one direct method. The selected source direction admits one
same-brand mixed invocation, so that narrow issuer cannot silently be reused
for multiple static rows.

The existing `ParserNormalSourcePlanSurfaceIssuerV1` already consumes prepared
static-parent rows as part of its seed, and the existing callable parameter
catalog owns parser-issued callable identities. A1 must connect those existing
products through one parser-owned relation set. It must not issue a second
semantic package or infer a relation downstream.

## Finite relation contract

For every admitted static parent, the source relation must co-seal:

```text
one ParserInvocationBrandV1
one SourceBoxDeclarationSiteV1 and static declaration syntax
member count and every parser member ordinal/kind
one direct SourceBoxMethodSiteV1 per admitted direct method
the matching CallableDeclarationIdentityV1
the matching complete parameter-catalog row
the final program slot that owns the parent
```

The set issuer must validate that parent paths are same-brand, unique, and
covered exactly once by final slots. Every direct method must have an empty
gate path, match its parent declaration and current member site, and match
exactly one callable identity and parameter row. `Field`, `InitBlock`, and
`StaticInitializer` remain an explicit unsupported partition until a separate
owner is selected. A prepared parent or callable row left unconsumed is an
orphan and rejects the set.

The issuer may retain the existing `ParserStaticBoxSourceSealV1` as the
per-parent structural seal, but the set relation must be issued once by the
parser postpass boundary. It must not clone or rebuild the existing rows from
AST names, method ordinals, or source text after the parser transaction.

## Named reject mapping

| Condition | Terminal | Required observation |
| --- | --- | --- |
| no static parent in an admitted mixed window | `SourceAuthorityUnavailable` | A0 window/seed missing |
| foreign parser brand or stale site | `IntegrityInvalid` | brand and declaration/member site disagree |
| duplicate parent path, member coordinate, or callable identity | `IntegrityInvalid` | more than one issuer row claims the same source relation |
| member count or final-slot coverage mismatch | `IntegrityInvalid` | cursor, slot, and parent rows disagree |
| missing callable or parameter row | `Incomplete` | a direct method has no parser-issued source row |
| unsupported member kind or non-direct method | `Outside` | explicit unsupported partition, no compatibility retry |
| prepared parent/callable row not consumed | `IntegrityInvalid` | orphan relation remains at set finish |

Every reject consumes the prepared set at this named parser terminal. It does
not downgrade a source-backed candidate to the AST-only compatibility route.

## Ordered design tasks

| Order | Task | Exit condition |
| --- | --- | --- |
| A1-D1 | Inventory current prepared static rows, final slots, callable identities and parameter rows | finite owner/caller/terminal table names every row and excludes unrelated families |
| A1-D2 | Choose the single set issuer boundary | postpass transaction is the only issuer; per-parent seal is not a second authority |
| A1-D3 | Define multi-parent/multi-method coverage checks | brand, path, member, slot, callable and parameter relations are exact and affine |
| A1-D4 | Freeze unsupported partitions and reject mapping | fields/init/static initializers, non-direct/gated/nested rows, and orphans have named terminals |
| A1-D5 | Define focused evidence and guard | positive two-parent/multi-method relation plus negative foreign/duplicate/missing/orphan cases are executable later without a new CI lane |

## Handoff and limits

After A1 design closes, the implementation path remains:

```text
CompletedParserPostpass
  -> static-parent set issuer
  -> ParserNormalSourcePlanSurfaceIssuerV1
  -> normal root/source plan
  -> callable catalog and semantic package
```

A1 does not admit the MixedProgram by itself. A0 source-window admission,
typed lineage transport (landed), body/import coverage, direct-call target and
result publication, production caller switching, and old compatibility-edge
retirement remain separate rows. No code, fixture, fallback, or new semantic
`Verified*`/`Prepared*` receipt is authorized by this card.

Keep `static_box_source.rs` and `normal_source_plan_surface.rs` below the
760-line design boundary and 800-line hard stop. If the relation set cannot be
issued from the existing parser transaction without reconstructing rows, stop
as `NoSafeSlice` and name the missing parser issuer rather than widening the
source window.

## Worker audit receipt — 2026-09-14

The read-only audit confirms that A1 remains design-only. The existing
`ParserStaticBoxParentSourceAuthorityIssuerV1` is the sole static-parent issuer
and `ParserNormalSourcePlanSurfaceIssuerV1` is its consumer; no second
semantic authority is needed. The finite implementation design is:

1. map every same-brand static parent to exactly one final program slot;
2. extend the issuer from one pure `StaticBox`/one-method cohort to multiple
   parents and direct methods while retaining explicit unsupported members;
3. co-seal each direct method by parent path, member site/ordinal, parser brand,
   callable identity, and exactly one parameter-catalog row;
4. reject duplicate, orphan, foreign-brand, missing-source, and coverage rows
   before `NormalSourcePlan` can become source-backed; and
5. leave callable body/constructor/direct-call handoff, publication, production
   switch, and old-edge retirement to later rows.

This audit leaves implementation permission false. A1 design must close the
finite inventory and exact reject mapping before any code or fixture is added.

## A1-D1 inventory closure — 2026-09-14

The source census fixes the finite relation boundary:

| Owner | Current edge | A1 treatment |
| --- | --- | --- |
| `source_seal/finalize.rs:196-201` | one call to `ParserStaticBoxParentSourceAuthorityIssuerV1::issue_once` with prepared static rows and callable rows | retain as the sole static-parent issuer; generalize its set contract |
| `callable_parameter_source/normal_source_plan_seed.rs:17-68` | transports projected slots plus prepared static-parent rows and rejects foreign/duplicate parent paths | retain as transport; it is not a second semantic issuer |
| `normal_source_plan_surface.rs:284-484` | consumes every prepared static row by final slot and checks callable identity/site, then rejects orphans | require exact agreement with the issuer's ready set before returning `Ready` |
| `postpass_envelope/normal_callable_program.rs:135-375` | stores the static-parent disposition and consumes it at named terminals; the normal surface currently does not inspect it | close this split by making the surface consume the issuer-owned set relation |
| `normal_source_plan_consumer.rs:321-339` and `normal_root_execution/issuer.rs:43-69` | borrow the already-issued surface rows and method relations | retain as downstream consumers; no path/name reconstruction |

This census exposes one design edge that cannot be hidden: the current
postpass disposition and the seed's prepared rows are two projections from the
same parser transaction, but only the seed rows reach the normal surface. A1
therefore selects the following single-authority join: the existing static
parent issuer remains the canonical set issuer, and the surface issuer must
consume its `Ready` set relation while validating the seed rows against it
before emitting `ParserBackedNormalSourcePlanBoundV1`. A seed row that is not
covered by the ready set, or a ready row not consumed by the seed/slot path, is
an affine integrity reject. The seed remains a transport carrier; it does not
issue a competing semantic receipt.

The finite callers are now one producer, one source-plan consumer, the existing
root/source-plan downstream consumers, and the explicit test/discard terminals.
No production target/result/publication owner is changed by A1. The current
source files are 437 lines (`static_box_source.rs`) and 648 lines
(`normal_source_plan_surface.rs`), below the 760-line design boundary; a
relation helper must be split before either file reaches 760.

This closes A1-D1 through A1-D5 as design work. The implementation slice is
created separately so its production edge, reject tests, and deletion set stay
reviewable.
