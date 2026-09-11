---
Status: closed__NoSafeSlice__GenericRewireDemandMissing__2026-09-11
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-GENERIC-REWIRE-D0
Parent: mir-callable-loop-phi-session-entry-i0-2026-09-11
---

# MIR-CALLABLE-LOOP-PHI-GENERIC-REWIRE-D0

## Six-line brief

```text
Decision: keep the broad GenericLoop rewire at design_stop until one exact
  source-bound handoff can replace the old Composer's value authority.
Source authority + canonical issuer: the claimed GenericLoopV1 Facts/Recipe
  owns source shape, BindingRef/site relations, and step placement; the
  CanonicalSsaFunctionSessionV2 with BindingSsaBuilderV1/PhiTxn owns physical
  ValueId, CFG, PHI, dominance, seal, and discard issuance.
Non-authority: generic_loop_pipeline, generic_loop_located_composer,
  CorePhiInfo, variable_map, phi_bindings, carrier_step_phis, names, and
  latest-value ordering are not semantic or physical SSA authority.
Fail-fast boundary: before Builder effects, require one owner/frame/site,
  complete operation/effect/continuation demand, canonical h_n/s_n relation,
  exact predecessor/dominance, and sealed publication; no old-Composer retry.
Smallest next slice: audit one existing GenericLoop production caller from
  Facts/Recipe claim through its physical terminal and prove/refute a
  builder-free handoff without AST reparse or a second semantic issuer.
Non-claims: no code, fixture, new semantic receipt, production switch,
  fallback, local completion, backend/OBJ/EXE, or legacy retirement.
```

## Census boundary

```text
existing GenericLoop caller
  -> GenericLoopV1 extraction/Facts and Recipe claim
  -> old Composer or source-port composer
  -> CorePlan/PlanLowerer physical terminal
includes: generic_loop_v1 facts, numeric/body-managed carrier roles,
          condition/body/step/After value flow, and the current callable
          source-port bridge where it reaches this composer vocabulary
excludes: CallableSingleLoop's already-accepted bounded session consumer,
          Generic G0, DirectAccum, nested/Outside routes, Dynamic, and
          compatibility-only legacy readers
```

## Observed authority split

The broad route starts from `try_extract_generic_loop_v1_facts` and
`GenericLoopV1Facts`. Those facts contain source syntax, body policy, carrier
role, and step placement, but no physical `ValueId`, block, PHI, or complete
operation/effect/continuation product. The generic normalizer then allocates a
`GenericLoopSkeleton`; `generic_loop_pipeline` and
`generic_loop_located_composer` create and transport name-keyed
`phi_bindings`, `carrier_step_phis`, `CorePhiInfo`, and `variable_map` state.

The source-backed callable bridge currently reaches the same CorePlan
vocabulary through
`RecipeComposer::compose_source_generic_loop_v1_recipe_with_port`.
That path is bounded and already covered by the selected CallableSingleLoop
acceptance, but it is not evidence that the broad GenericLoop Composer is a
canonical SSA owner. The future rewire must not simply pass
`CallableSemanticLoweringState` into the old Composer or wrap its maps.

The canonical physical owner is the existing function-scoped session and its
Binding SSA/CFG/PHI helpers. `CallableSemanticLoweringState` can provide the
source-owned BindingRef/site relation and exact completed values; it cannot be
replaced by the Composer's latest name map. The source Recipe also cannot be
treated as a complete physical demand until operation, effect, placement, and
JoinSig-derived After continuation are all present in one claimed product.

## Six-owner census

| Surface | Current owner | Authority allowed | Rewire disposition |
| --- | --- | --- | --- |
| source shape / step placement | `GenericLoopV1Facts` | source Facts only | preserve; no physical fields added |
| route-neutral callable source bridge | `CallableGenericLoopV1SemanticRecipeV1` | claimed source relation and existing Recipe | audit completeness before reuse |
| broad skeleton / carrier orchestration | `generic_loop_pipeline`, `carriers.rs` | compatibility CorePlan construction | not a canonical SSA issuer |
| body and condition reads | `generic_loop_body/*`, `generic_loop_step.rs` | existing port/legacy policy only | require one source-bound demand owner |
| physical values / CFG / PHI / seal | `CanonicalSsaFunctionSessionV2`, `BindingSsaBuilderV1`, `CanonicalCfgSessionV1`, `PhiTxn` | sole physical issuer | target owner |
| source completion / rebinding | `CallableSemanticLoweringState` | BindingRef and exact site/value relation | lend by scoped view; no map clone |

## Current decision

The next implementation permission is not yet present. The existing generic
Recipe/Facts products do not, by themselves, expose a complete neutral
operation/effect/continuation demand. `PlanBuildOutcome.recipe_contract` is a
structural optional and is normally absent on this source path. Therefore the
following are rejected as premature implementations:

- replacing `ValueId` reads with a latest-value scan;
- pairing `GenericLoopV1Facts` with Generic G0 or another family's operation
  product;
- passing `variable_map`, `phi_bindings`, or `carrier_step_phis` into the
  canonical session as if they were source authority;
- adding a semantic/session field to `RawInvocationChildPortV1`;
- calling the old Composer after a canonical candidate has been admitted.

The first task is a read-only caller census. It must identify one exact
production caller, the source product it claims, the physical terminal it
reaches, and whether all operation/effect/continuation rows already exist
before Builder effects. If the product is incomplete, the result is
`NoSafeSlice__GenericRewireDemandMissing` and the next card must design the
single source-bound demand producer; no partial adapter is allowed.

## Ordered tasks

1. Trace one caller from `nested_loop_depth1` or the selected source-port entry
   through `GenericLoopV1Facts`, the Composer, and the physical terminal. Record
   the exact caller class; do not generalize from tests or unused composers.
2. Inventory the existing claimed products and classify every field as source
   Facts, semantic Recipe, neutral operation/effect/continuation demand, or
   physical Builder state.
3. Prove or refute a builder-free projection to the existing common demand,
   preserving Recipe order, BindingRef/site identity, body/step placement,
   canonical `h_n -> s_n -> h_(n+1)`, and JoinSig-derived After relation.
4. Name the pre-effect rejects for missing/foreign owner, site, binding,
   operation, effect, continuation, predecessor, generation, or seal. Keep
   negative evidence mutation-discriminating.
5. Only after the design is accepted, taskize one bounded implementation slice
   and its 0/1/multiple iteration acceptance. Do not widen to nested, Dynamic,
   Generic G0, DirectAccum, or backend artifacts.

## Acceptance and retirement gate

This design stop closes only when one source authority, one demand issuer, one
canonical physical owner, one pre-effect verifier, and one production caller
are named. Acceptance must include a positive no-manual-ledger source fixture,
mutation rejects before physical effects, and a finite observation boundary.

The old GenericLoop Composer has no deletion set yet. Its deletion requires
the new consumer to be production-connected, the selected GenericLoop matrix
to pass, and all selected callers to be caller-zero. Generic G0, Dynamic,
non-callable, nested, and compatibility routes are outside that set.

## Caller census conclusion (2026-09-11)

The exact production GenericLoopV1 caller is:

```text
route_generic_loop_v1
  -> RecipeComposer::compose_generic_loop_v1_recipe
  -> generic_loop_pipeline::apply_generic_loop_v1_pipeline
  -> GenericLoopV1 carrier/body/condition/step orchestration
  -> CorePlan
  -> PlanVerifier::verify
  -> PlanLowerer::lower
```

`GenericLoopV1Facts` contributes source syntax, body policy, carrier role, and
step placement. `PlanBuildOutcome` contributes only optional Facts and a
structural `recipe_contract`, which is normally `None` for this route. The
Composer allocates the physical-looking skeleton and transports
`phi_bindings`, `carrier_step_phis`, `CorePhiInfo`, and `variable_map`; none is
a claimed operation/effect/continuation demand and none can be paired with
`CallableSemanticLoweringState` after the fact.

The existing common demand requires one complete source-bound Core,
operation/effect evidence, and JoinSig-derived After continuation before a
canonical session opens. The census found no builder-free product supplying
those rows for this broad caller. Therefore this family closes as
`NoSafeSlice__GenericRewireDemandMissing`; the old Composer remains an
explicit compatibility owner and is not deleted or promoted. The next
independent bounded row is
`MIR-CALLABLE-LOOP-LOCAL-COMPLETION-HANDOFF-R0`.
