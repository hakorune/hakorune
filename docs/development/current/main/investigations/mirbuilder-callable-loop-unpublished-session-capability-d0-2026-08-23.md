Status: Design stop; conditional design only, no implementation authorized
Task: MIR-CALLABLE-LOOP-UNPUBLISHED-SESSION-CAPABILITY-D0
Date: 2026-08-23
Priority: constrain the source-aware physical adapter to the existing unpublished function transaction
Parent: MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-D0
Current execution row: MIR-CALLABLE-LOOP-UNPUBLISHED-SESSION-CAPABILITY-D0
CurrentCard: docs/development/current/main/investigations/mirbuilder-callable-loop-unpublished-session-capability-d0-2026-08-23.md
NextCard: none until this Decision is accepted
---

# Callable Loop unpublished-session capability D0

## Six-line brief

Decision: Conditional Accept. Reuse the existing `CanonicalFunctionLoweringSessionV1` as the sole unpublished physical-effect owner; create only a private scoped facade minted by that session. Do not create a second Builder transaction or a wrapper constructible from a bare `&mut MirBuilder`. The source-aware adapter may be changed only after its session handoff and discard terminal are fixed as one bounded slice.
Source authority + canonical issuer: `CallableGenericLoopSourceFactsIssuerV1` issues the source-backed Facts once, `CallableGenericLoopV1SemanticRecipeIssuerV1` issues the semantic Recipe once, and `CanonicalFunctionLoweringSessionV1` owns the unpublished function, caller snapshot, and discard boundary. `CallableGenericLoopV1PhysicalAdapterV1` is the sole named physical consumer. No new semantic authority is issued here.
Non-authority: a bare `&mut MirBuilder` passed from `RawLoopChildEntryPortV1`, `RecipeComposer` mutation, `PlanBuildOutcome`, `CanonicalLoopFacts`, `PlanVerifier`, `variable_map` snapshots, `ValueId`, AST, route names, and a wrapper constructible without an existing open session.
Fail-fast boundary: before the named source-aware adapter can call the physical RecipeComposer, an open canonical function session must be present and owned by the caller. Every adapter/composer/verifier/lowerer error remains inside that unpublished session and reaches its sole discard terminal; no module publication, retry, or fallback is allowed.
Smallest next slice: close the production caller census and specify the exact session handoff from the raw callable Loop entry to the named adapter, reusing `builder_view_mut_for_lowering()` only through the open session. The first implementation cell may narrow the adapter/port signatures and add effect-zero-on-reject evidence; it must not claim a pure symbolic plan.
Non-claims: no `SymbolicCorePlan<LoopValueKey>` rewrite, no PlanVerifier-before-all-effects guarantee, no general Builder capability redesign, no legacy route migration, no ordinary Outside consumer, no publication change, no new Loop cohort, no performance work.

## Current problem

The current production route is:

```text
Ready source Facts
  -> claim_all
  -> semantic Recipe
  -> CallableGenericLoopV1PhysicalAdapterV1::lower(&mut MirBuilder, recipe)
  -> RecipeComposer::compose_source_generic_loop_v1_recipe(&mut MirBuilder, ...)
  -> PlanVerifier::verify(&plan)
  -> PlanLowerer::lower(&mut MirBuilder, plan, ...)
```

`RecipeComposer` is not a pure conversion: it allocates a GenericLoop skeleton,
issues physical values, and mutates Builder state while constructing the plan.
`with_saved_variable_map_typed` restores only selected local maps; it is not the
unpublished function transaction. Therefore the names `compose -> verify ->
lower` currently hide a pre-verification physical effect.

The existing `CanonicalFunctionLoweringSessionV1` already owns the correct
outer rollback boundary. Its `builder_view_mut_for_lowering()` is available
only while the session is open, and `discard_unpublished()` restores the caller
and clears the unpublished function. The design question is how to pass that
existing capability through the one production callable Loop route without
creating a second session or letting a bare Builder remain an accepted entry.

## Candidate boundary

The preferred direction is a session-owned private facade, not a new authority:

```text
open CanonicalFunctionLoweringSessionV1
  -> source Facts / semantic Recipe (already issued)
  -> existing session mints UnpublishedCallableLoopLoweringView
  -> named adapter receives only that scoped view + Recipe
  -> compose / verify / lower stay unpublished
  -> success continues to existing draft close
  -> any error calls session.discard_unpublished()
```

The facade contains no independent `MirBuilder`, CFG, ValueId, collector, or
publication state. It exposes no generic mutable Builder getter and cannot be
constructed anywhere except the existing session owner. The adapter must not
open or close the session itself, and it must not return a `MirBuilder`,
`CanonicalFunctionLoweringSessionV1`, or partially published function. The
production caller census must identify the one current
`RawInvocationChildPortV1 -> PreparedLocatedRawLoopChildEntryV1::lower_v1`
edge and the outer session that already encloses it. If that enclosing session
cannot be named without opening a second session, this slice is `NoSafeSlice`.

F3 does not claim that `PlanVerifier` is already no-effect: the current
composer allocates a skeleton and mutates the unpublished Builder before the
verifier runs. The bounded claim is only that those effects are confined to
the existing unpublished session and failures reach its sole discard terminal.

## Worker Decision — Dirac (read-only)

The worker found one named production adapter caller at
`raw_loop_child_entry.rs:236` and confirmed that the existing
`CanonicalFunctionLoweringSessionV1` is the correct sole unpublished owner.
The worker conditionally accepts only a private facade minted by that session:

```text
CanonicalFunctionLoweringSessionV1
  -> UnpublishedCallableLoopLoweringView
  -> CallableGenericLoopV1PhysicalAdapterV1
```

The facade must not contain an independent Builder/session/CFG/ValueId or
publication state, and must not expose arbitrary
`builder_view_mut_for_lowering()`. The worker explicitly rejects claiming
“PlanVerifier-before-effect”; that is a later pure symbolic-plan or prepared
physical-plan design. F3 may claim only unpublished-effect confinement and
outer-session discard on failure.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| Resolver semantic state | exact source rows and bindings | physical blocks, ValueIds, route repair |
| `CallableGenericLoopSourceFactsIssuerV1` | one Facts/Recipe issuance | Builder, MIR, publication |
| `CanonicalFunctionLoweringSessionV1` | unpublished function, caller snapshot, discard/close | source semantics, route selection |
| `CallableGenericLoopV1PhysicalAdapterV1` | one consumer handoff and physical sequence | second session, source re-observation, fallback |
| `RecipeComposer` | existing CorePlan construction mechanics | source authority, publication, retry |
| `PlanVerifier` | existing plan validation | rollback, source re-resolution |
| `PlanLowerer` / sole writer | existing physical lowering | meaning reclassification, publication |

## Finite state table

| State | Owner | Effect | Allowed next | Forbidden |
| --- | --- | ---: | --- | --- |
| `RecipeReady` | semantic Recipe owner | none | session handoff | bare Builder adapter call |
| `CapabilityBound` | existing session + private facade | none | composing | second session or free Builder |
| `Composing` | named adapter | unpublished only | `ComposedUnverified` or discard | module publication |
| `ComposedUnverified` | private adapter state | unpublished only | `Verified` or discard | reusable public receipt |
| `Verified` | private adapter state | unpublished only | `LoweredUnpublished` | source re-resolution |
| `RejectedUnpublished` | session discard terminal | cleared/unpublished | outer error | retry/fallback |
| `LoweredUnpublished` | session + existing draft close | unpublished MIR | existing close/commit | free Builder escape |
| `NoSafeSlice` | design owner | none | stop/review | guessed wrapper/default route |

## Acceptance evidence required before implementation

```text
one named production caller of the source-aware adapter
one already-open CanonicalFunctionLoweringSessionV1 at that caller
adapter accepts the private scoped facade, not bare &mut MirBuilder
zero facade constructors outside CanonicalFunctionLoweringSessionV1
zero second function session opened by the adapter/port
error path -> existing discard_unpublished() exactly once
success path -> existing draft-close owner, no publication in adapter
outside terminal remains Builder-effect zero
focused mutation probe proves rejected compose/verify/lower leaves no published module
source/Facts/Recipe issuance count remains one
Composer, PlanVerifier, and PlanLowerer each run once
```

The first implementation slice must be a signature/ownership boundary and its
focused reject evidence. It must not pretend that `PlanVerifier` has become a
no-effect verifier; that remains a later pure-plan or prepared-physical-plan
design.

## Task sequence after Decision acceptance

```text
T0  caller census: identify the enclosing CanonicalFunctionLoweringSessionV1
    and prove there is exactly one Ready adapter edge
T1  private facade: add one move/borrow-scoped
    UnpublishedCallableLoopLoweringView minted only by that session
T2  port cutover: thread the facade through the callable Ready edge; remove
    the adapter's bare &mut MirBuilder entry and keep Outside terminal-only
T3  discard evidence: route every compose/verify/lower error to the existing
    outer discard terminal exactly once; do not add retry or fallback
T4  focused proof: positive Ready path, reject-after-compose mutation probe,
    no-publication assertion, one-call structural guard, source-size/diff checks
R0  closeout: update module README/reference/card/CURRENT_STATE, commit/push,
    and stop before pure-plan work
```

T1/T2 are not authorized while this card remains `design_stop`; they require
an accepted Decision with the exact session handoff named in T0.

## NoSafeSlice conditions

Stop here if any of these holds:

```text
the current Ready caller cannot name an enclosing unpublished session
the adapter needs a new session to obtain a Builder view
the facade exposes arbitrary `&mut MirBuilder` or can be copied, stored, or returned to another consumer
the adapter error cannot reach the existing discard terminal exactly once
RecipeComposer requires a published module or a second source observer
the port change would also activate the Outside ordinary lane
zero physical effect before PlanVerifier is required to make this slice safe
the only available proof is variable-map restoration or an AST/MIR re-scan
```

If `PlanVerifier` must become effect-free before the session handoff can be
safe, stop and open a separate pure `SymbolicCorePlan` design. Do not smuggle
that larger refactor into this bounded capability slice.

## Parked follow-ups

```text
MIR-CALLABLE-LOOP-PURE-SYMBOLIC-CORE-PLAN-D0
MIR-CALLABLE-LOOP-STRICT-PLAN-VERIFIER-D0
MIR-CALLABLE-LOOP-I9-TRANSACTION-HARDENING-D0
builder.rs barrel/experimental namespace cleanup
compile-time performance measurement gate
```
