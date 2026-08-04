# JoinIR Loop all-route premise reset — D0 consultation

Status: Historical consultation; corrected closeout is Decision B-prime
Date: 2026-08-04
Current row: `JOINIR-LOOP-ALL-ROUTE-PREMISE-RESET0-D0`
Decision: `B-prime` — exclude typed-unreachable raw/reference profiles, retain
the located Loop migration fence, and reject universal semantic ingress.

The question/evidence sections below preserve the premise presented to the
consultation. They are historical input, not current caller authority. The
corrected closeout at the end supersedes the stale `C / NoSafeSlice` answer
that was added after B-prime and normalized-shadow retirement had landed.

## Context

Two prior Loop design rows closed as `NoSafeSlice`.

```text
JOINIR-LOOP-RECIPE-COREPLAN0-D0
  -> existing route selection is operational and multi-candidate;
     CorePlan is already physical; normalized shadow mutates and falls back

JOINIR-LOOP-LOGICAL-INTERFACE0-D0
  -> no all-caller producer can issue ordered logical inputs, carrier/exit
     mappings, continuation identity, and result disposition
```

The attempted final prerequisite also closed as `NoSafeSlice`:

```text
JOINIR-LOOP-ROOT-NEUTRAL-BINDING-SNAPSHOT0-D0
  -> no root-neutral producer can cover the full atomic caller set before
     Builder effects
```

The question is now about the premise, not another Loop recipe type. A
selected-only Loop product would leave the shared normalized-shadow mutation
route or raw/reference compatibility portal alive, so it is not an acceptable
final-pipeline cutover.

## Current caller evidence

| Atomic caller | Exact Loop source | Logical binding identity | Carrier / exit / continuation | Current boundary |
| --- | --- | --- | --- | --- |
| Function/Lambda resolved owner | Yes | `BindingRefV1` for resolved declarations/uses | No; Loop facts are scope/region only | facts are not consumed by ordinary Loop lowering |
| selected Script | sparse Program ordinal window | No | No | `ScriptLexicalCoreV1` defers `Loop` before child descent |
| selected raw invocation | Loop, condition, body receipts | No | No | `PreparedLocatedRawLoopChildEntryV1` discards receipts into `_located_receipts` and sends raw AST to the route |
| shared legacy block suffix | no stable source/owner product | No | No | clones physical `BTreeMap<String, ValueId>` |
| raw/reference legacy | No | No | No | `RawLegacyChildLoweringPortV1` destructures AST and calls raw Loop routing |

The decisive absence is raw/reference. It has neither source lineage nor a
semantic binding environment. Giving it one would require a new raw semantic
ingress; that is a material route change, not a neutral Loop adapter.

## Existing but insufficient vocabulary

The repository already contains `BindingRefV1`, Loop scope/region facts, and
`SourceBindingSiteV1::LoopBinder`. These do not establish a producer:

```text
SourceBindingSiteV1::LoopBinder
  -> source projection explicitly rejects it as not activated

Function/Lambda Loop facts
  -> exact lexical/control facts
  -> no ordered dataflow inputs, carrier-to-exit mapping, continuation plan,
     or result disposition

selected Script
  -> source window exists
  -> Loop semantic resolution is deliberately inactive
```

The current normalized route remains physical:

```text
String carrier name
-> host variable_map lookup
-> ValueId / block / PHI construction
-> phase-only Normalized -> Structured converter snapshot
-> operational `None` fallback to another route
```

That cannot be relabelled as a verified semantic product.

## Non-negotiable final-pipeline contract

Any accepted direction must preserve all of the following.

```text
- one physical Loop lowerer; no second resolver/lowerer
- no fabricated source identity or name/ValueId-based logical recovery
- source facts, route selection, and recipe verification are Builder-mutation-free
- `ValueId`, CFG blocks, PHIs, MIR mutation, and publication belong only to
  the physical terminal
- one selected route or typed source decline before mutation; no candidate
  execution followed by `None` fallback
- verification/lowering failure never retries a raw or normalized route
- both normalized-shadow mutation entries must retire in the same final cutover
- raw/reference is unchanged unless it is explicitly covered by the same
  all-route owner switch
- ordinary VM remains the only VM execution owner
- Loop grammar, user diagnostics, and result semantics do not change
```

The eventual logical product, if one is possible, needs all of this without
physical IDs:

```text
exact Loop / condition / body source sites
BindingRef-based ordered inputs
logical carrier -> exit slots
continuation identity
result / exit disposition
```

## Candidate decisions

### A. Universal resolved semantic ingress is a prerequisite

Declare that all Loop callers, including raw/reference and the shared block
driver, must first receive one root-neutral source/binding semantic ingress.

This must not be a second resolver. Please name the single source of truth,
the migration order, and how raw/reference behavior remains stable during the
migration. Also state whether this is sufficiently bounded to begin as a
Refactor Series rather than an unbounded compiler rewrite.

### B. Redefine atomic Loop caller membership

Allow an earlier selected-only Loop cutover only if the excluded callers can be
shown to be behind a separately typed, final retained operation with an
independent all-route owner and a real release condition.

It is not enough to say "raw/reference is legacy" or to keep generic
`script_root(())`, `variable_map`, or normalized shadow alive indefinitely.
Name the exact retained owner, ingress, consumer, and final deletion path.

### C. Reject the current Loop final-pipeline replacement premise

If neither A nor B can meet the contract, explicitly reject the current Loop
replacement premise and name the architectural decision that the repository
owner must make. Do not invent a partial Loop I0.

## Questions

1. Which candidate is correct today: A, B, or C? State the decisive reason.
2. Is universal semantic ingress genuinely required for final-pipeline Loop
   ownership, or can raw/reference be excluded without leaving an untyped
   compatibility edge? Give the exact proof obligation either way.
3. If A is accepted, name the smallest root-neutral ingress product, unique
   issuer, and consumer boundaries. Explain how it avoids a second resolver
   and does not activate Script Loop Complete prematurely.
4. If B is accepted, name the all-route typed retained operation for excluded
   callers and its sunset condition. Explain how both normalized-shadow
   mutation routes still reach zero.
5. Does the current `SourceBindingSiteV1::LoopBinder` rejection mean that a
   language/source-binding projection decision is required first? If so,
   identify that separate decision and what it must not claim.
6. What single focused evidence set decides the chosen premise before any Loop
   S0/I0: caller membership, binding/source coverage, no Builder effects,
   raw/reference parity, and old-edge reachability?
7. If the premise cannot be closed now, give the precise `NoSafeSlice`
   closeout and the owner decision required from the project rather than a
   new compatibility adapter.

## Explicit non-claims

```text
- Do not implement a Loop row in this consultation.
- Do not reactivate the retired JoinIR VM bridge or add another converter.
- Do not add a Script-only loop resolver, a raw-only resolver, or a
  name-based binding recovery table.
- Do not move ValueId, CFG, PHI, ABI, or publication authority into facts.
- Do not treat existing Function/Lambda facts as proof of all-caller coverage.
- Do not change raw/reference, diagnostics, Loop grammar, or result semantics
  merely to make an answer fit.
```

## Desired answer format

```text
Decision: A / B / C / NoSafeSlice
Premise and all-route membership:
Required source/binding authority:
Unique issuer and physical consumer:
Compatibility / retention contract, if any:
Atomic old-edge deletion path:
Failure / retry / publication contract:
Focused proof set:
First executable row, or exact owner-level stop:
```

## Corrected closeout — 2026-08-04

Decision: `B-prime`.

The former `C / NoSafeSlice` closeout was a documentation regression. It
over-counted a profile-blind helper as a raw/reference compilation profile and
reintroduced normalized-shadow mutation after that authority had been deleted.

```text
raw public / raw VM-reference NarrowV1:
  owned source exists
  Loop and LoopRange reject during body-recipe projection
  rejection occurs before physical Builder open
  production Loop reachability = 0

RawLegacyChildLoweringPortV1:
  generic lowering capability
  compilation profile / source authority = no

normalized-shadow mutation:
  direct entry / suffix entry / Plan / Execute / retry / phase bridge = 0
  retained normalized-shadow observer mutates no MIR

located RawInvocation Loop:
  exact parent / LoopCondition / LoopBodyRoot receipts exist
  PreparedLocatedRawLoopChildEntryV1 remains the named migration fence
  receipts are still erased before the legacy route, so M11 is not complete
```

Universal raw/reference semantic ingress is therefore rejected. The active R4
fence `RAW-LOCATED-LOOP-ROUTE-SOURCE-HANDOFF-SUNSET-001` remains the exact
retained operation; it retires only when the same located product is consumed
by the verified Loop plan and the source-erasing terminal reaches zero.

This correction does not authorize M10b. M7 five-family closure, M8 all-19
producer closure, M9 host parity, and Generic D2 still precede the atomic
scheduler/Retry/old-PHI cutover. The first executable row is the caller-zero
`JOINIR-LOOP-TRUE-BRANCH-EXIT-CLOSURE0-M7-S2-A-S0` logical JoinSig slice.

No new resolver, raw profile, Builder path, route, retry, fallback, grammar,
IR, runtime, or diagnostic behavior is introduced by this closeout.
