---
Status: active__Implementation__GenericG0CanonicalIssuer__2026-09-11
Task: LOOP-G0-CANONICAL-ISSUER-I0
Date: 2026-09-11
Priority: issue one source-backed Generic G0 plan and bind it to the existing Single package lifecycle
Parent: mirbuilder-loop-g0-production-terminal-d0-2026-09-11
NextCard: LOOP-G0-PRODUCTION-TERMINAL-I1
---

# Generic G0 canonical issuer I0

## Six-line brief

```text
Decision: add one GenericG0 canonical preflight arm and carry it through the existing BindingSsaTrivial/Single package boundary.
Source authority + canonical issuer: the existing resolver input and the accepted GenericG0 policy handoff; one issuer co-seals source products, G0 declaration facts, lifecycle identity, and the plan payload.
Non-authority: physical MIR, physical IDs, generic_g0_physical_emitter_session, route_loop, registry observation, AST re-scan, and any fallback.
Fail-fast boundary: source/header/arity/owner/origin compatibility is rejected before bind, Builder/session mutation, route dispatch, or physical lowering.
Smallest next slice: issue the GenericG0 plan, add the exact package arms, prove the Single mapping, and add focused positive/negative/no-reentry evidence.
Non-claims: no physical lowerer, completion drain, publication, legacy deletion, all-family selection, or source-to-exe completion in this card.
```

## Authorized implementation cells

Only the following bounded cells are authorized:

1. `src/mir/compiler/capability/first_family_plan.rs`: add the one
   `CanonicalLoopFamilyPlanV1::GenericG0` payload and preserve the external
   `TrivialBindingSsa` brand.
2. `src/mir/compiler/source_bound_plan.rs` and the smallest required package
   consumer: carry the plan as the existing `ExactCanonicalPreflightPlanV1::Loop`
   shape and keep `route()` equal to `BindingSsaTrivial`; do not add a G0 token
   or continuation variant.
3. `src/mir/compiler/capability.rs` and the named production preflight caller:
   select G0 only after NestedPredicate/DirectAccum and before ordinary
   Trivial/A+, using the already accepted source handoff. A known non-G0 shape
   declines; source identity or coverage failure rejects.
4. The canonical package bind/seal consumer may accept the co-sealed G0
   declaration/lifecycle view, but it must not reissue either header, inspect
   MIR, or open a Builder session. Physical `consume_parts` and publication
   stay outside I0 unless the exact pre-bind boundary remains unchanged.
5. Add or update one reusable guard plus focused tests. Tests must call the
   production issuer/package boundary, not the test-only emitter session.
6. Update the owning module README and the canonical Loop reference only for
   this source-to-package contract.

If any cell requires a new semantic owner, a second continuation/token, a
physical session, or guessed source identity, stop and return this card to
`NoSafeSlice`.

## Co-seal contract

The issuer retains the existing source parent products and the two existing
views in one move-only plan. `VerifiedGenericG0TopLevelDeclarationHeaderV1`
supplies G0 semantic declaration/shell facts. `VerifiedResolvedOwnerHeaderV1`
supplies package identity and the existing Single manifest/lifecycle. The
issuer checks owner, source identity, canonical name, and explicit parameter
arity once, then downstream borrows the plan. No consumer calls either header
issuer again and no package field is reconstructed from MIR, token, arity
count, or symbol text.

The external map is fixed:

```text
CanonicalLoopFamilyPlanV1::GenericG0
  -> ExactCanonicalPreflightPlanV1::Loop
  -> CanonicalSourceRouteV1::BindingSsaTrivial
  -> ModuleInvocationFamilyV1::BindingSsaTrivial
  -> CanonicalSourceContinuationV1::Single
```

This is a source-bound handoff only. I0 does not claim that the plan can yet
produce a physical `MirFunction` or reach `publish_once`.

## Failure and no-reentry contract

The following are typed rejection paths and must occur before `bind` or any
Builder/session effect:

- foreign owner, source origin/kind, loop site, frame, scope/region, or body
  root;
- declaration/lifecycle header mismatch or explicit parameter arity drift;
- missing G0 handoff/product/coverage or a non-G0 selection passed to the G0
  issuer;
- a second source-parent/header issuer, `route_loop` re-entry, ordinary A+
  fallback after source-integrity failure, or test-only emitter invocation.

Known non-G0 shapes may decline into the pre-existing ordinary profile only at
the source-selection boundary. Once G0 has been selected, no lowerer failure
may reclassify it as A+ or a legacy Generic route.

## Focused acceptance

```text
positive exact GenericG0 source -> one GenericG0 plan -> BindingSsaTrivial/Single package
known non-G0 source -> ordinary decline, no G0 package bind
foreign/mismatch source -> typed reject before bind/session/route
selected G0 -> route_loop = 0, test emitter session = 0, second issuer = 0
source/header identity checks -> one canonical comparison owner
```

Required checks are the focused Rust tests for the issuer/package boundary,
the reusable structural guard, `git diff --check`, pointer guard, and source
size counts. Cargo commands are serialized and use the repository quick
profile with at most two jobs on the 16 GiB development machine.

## Closeout requirements

Close I0 only with changed-file inventory, positive/negative test results,
guard result, source-size result, README/reference update, commit SHA, and
pushed remote state. If physical lowering or source-to-exe evidence is still
missing, leave `LOOP-G0-PRODUCTION-TERMINAL-I1` as the next active card and do
not claim MIRBuilder or Generic G0 production completion.
