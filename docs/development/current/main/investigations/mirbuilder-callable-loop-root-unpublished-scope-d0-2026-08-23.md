Status: Design stop; corrected authority and bounded implementation task map
Task: MIR-CALLABLE-LOOP-ROOT-UNPUBLISHED-SCOPE-D0
Date: 2026-08-23
Priority: bind the Ready source-aware Loop adapter to the existing unpublished root candidate
Parent: MIR-CALLABLE-LOOP-OUTSIDE-OBSERVED-CLASS-D0
Current execution row: MIR-CALLABLE-LOOP-ROOT-UNPUBLISHED-SCOPE-D0
CurrentCard: docs/development/current/main/investigations/mirbuilder-callable-loop-root-unpublished-scope-d0-2026-08-23.md
NextCard: none until this Decision is accepted
---

# Callable Loop root unpublished scope D0

## Six-line brief

Decision: Conditional Accept. The live root Ready path is owned by the existing `ModuleBuilderInvocationSessionV1`, whose `candidate: MirBuilder` is the unpublished root candidate. Co-seal a private root lowering scope from that existing session and the same-root `ModuleDraftCollectorV1`; do not open `CanonicalFunctionLoweringSessionV1` for this root path and do not create a second module session.
Source authority + canonical issuer: `CallableGenericLoopSourceFactsIssuerV1` issues source Facts once, the semantic Recipe issuer issues the Recipe once, and `ModuleBuilderInvocationSessionV1` owns the unpublished root candidate/discard boundary. `CallableGenericLoopV1PhysicalAdapterV1` remains the sole named physical consumer. The scope aggregates existing authority; it issues no new semantic meaning.
Non-authority: bare `&mut MirBuilder` at the adapter boundary, `ModuleLoweringInvocationV1` (disconnected shell), `ModuleLoweringPortV1` (collector transport only), `CanonicalFunctionLoweringSessionV1` (child function-draft owner for this path), `RecipeComposer` mutation, `PlanBuildOutcome`, `PlanVerifier`, `ValueId`, AST, route names, variable-map snapshots, and any wrapper constructible from a bare Builder.
Fail-fast boundary: before the Ready adapter enters the physical RecipeComposer, the caller must bind the existing root session scope. Composer/PlanVerifier/PlanLowerer errors remain unpublished and must return through the root lifecycle rejection path before collector drain/commit; no retry, fallback, or publication occurs inside the adapter.
Smallest next slice: add a private root-scoped borrow facade that co-seals the existing `ModuleBuilderInvocationSessionV1` candidate, the same-root collector, and invocation brand; thread it only through the Ready physical edge; prove exactly one collector drain/commit on success and zero drain on failure. Keep Facts→Recipe unchanged and do not claim a pure symbolic plan.
Non-claims: no `SymbolicCorePlan<LoopValueKey>`, no PlanVerifier-before-all-effects guarantee, no general Builder capability redesign, no ordinary Outside consumer, no legacy route migration, no publication protocol change, no new Loop cohort, no parser/source re-observation, and no performance work.

## Top-down caller audit

The actual root path is:

```text
ModuleBuilderInvocationSessionV1::complete_normal_default_program_root_catalog_lifecycle
  -> with_builder_and_pinned_text_invocation_binding
  -> MirBuilder::lower_normal_default_program_root_after_catalog_install_v1
  -> lower_program_root_after_catalog_install_v1
  -> local ModuleDraftCollectorV1
  -> ModuleLoweringPortV1
  -> RawInvocationChildPortV1
  -> RawLoopChildEntryPortV1::lower_loop(&mut MirBuilder, ...)
  -> Ready source-Facts issuer
  -> claim_all()
  -> semantic Recipe
  -> CallableGenericLoopV1PhysicalAdapterV1::lower(builder, recipe)
  -> RecipeComposer
  -> PlanVerifier
  -> PlanLowerer
  -> collector drain
  -> collector commit
```

The root candidate is created at `ModuleBuilderInvocationSessionV1` and is
kept separate from the live Builder until external commit. The local collector
is currently created in `lower_program_root_after_catalog_install_v1`, so the
first scope slice must co-bind the candidate and that collector under the same
invocation brand. `ModuleLoweringInvocationV1` is a disconnected/future shell
and is not allowed to become a second live owner.

`CanonicalFunctionLoweringSessionV1` is used by child method/function draft
lowering. It does not enclose this root Ready Loop. Opening it here would create
an unrelated function transaction and is `NoSafeSlice`.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| resolver/source package | exact source rows, binding relations, Facts | Builder, collector, physical IDs |
| `CallableGenericLoopSourceFactsIssuerV1` | one source-Facts issuance | physical lowering, publication |
| semantic Recipe issuer | one AST-free Loop Recipe | source re-scan, module commit |
| `ModuleBuilderInvocationSessionV1` | unpublished root candidate, caller snapshot, external commit/discard | source semantics, route selection |
| same-root `ModuleDraftCollectorV1` | child draft admissions and drain/commit | Builder transaction ownership |
| private root lowering scope | borrow-only co-seal of the two existing owners + brand | independent state, publication, second session |
| `CallableGenericLoopV1PhysicalAdapterV1` | one Ready consumer sequence | session creation, source re-observation, fallback |
| `RecipeComposer` | existing physical plan construction mechanics | source authority, commit |
| `PlanVerifier` | existing plan validation | rollback, source re-resolution |
| `PlanLowerer` / physical writer | existing unpublished MIR effects | meaning reclassification, publication |

## Corrected API shape

The exact names remain implementation-level, but the ownership shape is fixed:

```rust
struct UnpublishedCallableLoopRootScopeV1<'scope> {
    // borrows the existing ModuleBuilderInvocationSessionV1 candidate
    // and the same-root ModuleDraftCollectorV1
    // fields private; no independent Builder/collector/session
}

impl ModuleBuilderInvocationSessionV1 {
    fn with_callable_loop_root_scope<R>(
        &mut self,
        collector: &mut ModuleDraftCollectorV1,
        use_scope: impl FnOnce(UnpublishedCallableLoopRootScopeV1<'_>) -> R,
    ) -> R;
}

impl CallableGenericLoopV1PhysicalAdapterV1 {
    fn lower(
        scope: UnpublishedCallableLoopRootScopeV1<'_>,
        recipe: CallableGenericLoopV1SemanticRecipeV1<'_>,
    ) -> Result<ValueId, String>;
}
```

This is a contract sketch, not an implementation command. The facade must not
expose a generic mutable Builder getter. It may expose only the one named
operation needed by the existing RecipeComposer/PlanLowerer sequence, or keep
the candidate access entirely inside the adapter module. It cannot be copied,
stored after the callback, returned to another consumer, or constructed from a
bare `&mut MirBuilder`.

The collector and candidate must be borrowed from the same root lifecycle. A
second `ModuleBuilderInvocationSessionV1`, `CanonicalFunctionLoweringSessionV1`,
or `ModuleLoweringInvocationV1` is forbidden.

## Effect and failure boundary

The current composer allocates a GenericLoop skeleton and can mutate the
unpublished candidate before `PlanVerifier`. Therefore this slice does **not**
claim that verification is effect-free:

```text
Ready Recipe
  -> root scope bound
  -> compose / verify / lower
  -> unpublished candidate only
  -> collector drain/commit only after the whole root path succeeds
```

Any error before collector drain returns through the existing rejected root
lifecycle. The candidate is retained for the outer rejection owner and is never
published. The adapter adds no retry or fallback.

## Finite state

| State | Effect | Allowed next |
| --- | ---: | --- |
| `RootSessionOpen` | none | `RootScopeBound` |
| `RootScopeBound` | none | `ReadyRecipe` |
| `ReadyRecipe` | none | `UnpublishedCompose` |
| `UnpublishedCompose` | candidate only | `Verified` or reject |
| `Verified` | candidate only | `LoweredUnpublished` |
| `LoweredUnpublished` | unpublished MIR | collector drain |
| `CollectorDrainPrepared` | unpublished module draft | one collector commit |
| `Committed` | one external candidate commit | terminal |
| `Rejected` | no published effect | discard/drop candidate |
| `NoSafeSlice` | none | design stop |

## Bounded task sequence

```text
T0  caller census (complete): record the root call chain above and prove the
    root owner is ModuleBuilderInvocationSessionV1, not a child function session.

T1  root scope design: define the private borrow facade that co-seals the
    existing candidate + same-root collector + invocation brand. No new session,
    no independent state, no generic Builder getter.

T2  Ready port cutover: thread the scope through the one Ready physical edge;
    remove the adapter's bare-Builder entry. Keep Outside terminal-only and the
    Facts/Recipe issuer counts at one.

T3  failure terminal: verify every composer/verifier/lowerer error returns
    before collector drain; preserve the existing root rejected-session owner;
    add no fallback/retry.

T4  focused evidence: one Ready positive path, one injected/rejected physical
    path with no collector drain/commit, one success with exactly one drain and
    one commit, Outside effect-zero regression, caller/constructor guards, and
    source-size/diff checks.

R0  closeout: update the builder README, this card, CURRENT_STATE, focused test
    receipt, and pointer; commit/push. Stop before pure-plan work.
```

## Acceptance and guards

```text
root Ready adapter caller count = 1
adapter bare `&mut MirBuilder` entry = 0
root scope constructor outside ModuleBuilderInvocationSessionV1 = 0
CanonicalFunctionLoweringSessionV1 opened by root Ready path = 0
ModuleLoweringInvocationV1 introduced into live root path = 0
source Facts issuer = 1
semantic Recipe issuer = 1
collector drain = 1 on success
collector commit = 1 on success
collector drain/commit = 0 on reject
Ready -> lower_loop_or_freeze_v1 = 0
Ready -> legacy fallback/retry = 0
Outside Builder effect = 0
```

Focused evidence may observe unpublished candidate mutation, but it must prove
that no module reaches external commit after a rejected Ready physical path.
The test must not use a second source scan or reconstruct Facts/Recipe from AST.

## NoSafeSlice

Stop and reopen design if:

```text
candidate and collector cannot be co-bound under the same invocation brand
the facade exposes or returns arbitrary mutable Builder access
the Ready path needs a second module/function session
adapter failure can bypass the root rejected-session owner
collector drain can occur before the full Ready physical path succeeds
PlanVerifier-before-any-physical-effect is required for this cell
source Facts/Recipe must be reissued or AST must be rescanned
Outside would need to enter this consumer to make the slice work
```

## Parked follow-ups

```text
MIR-CALLABLE-LOOP-PURE-SYMBOLIC-CORE-PLAN-D0
MIR-CALLABLE-LOOP-STRICT-PLAN-VERIFIER-D0
MIR-CALLABLE-LOOP-I9-TRANSACTION-HARDENING-D0
builder.rs production/experimental barrel cleanup
compile-time performance measurement gate
```
