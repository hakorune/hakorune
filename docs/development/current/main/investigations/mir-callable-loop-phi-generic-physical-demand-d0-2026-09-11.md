---
Status: design stop; implementation not opened
Date: 2026-09-11
Decision: MIR-CALLABLE-LOOP-PHI-GENERIC-PHYSICAL-DEMAND-D0
Parent: mir-callable-loop-phi-session-entry-i0-2026-09-11
---

# MIR-CALLABLE-LOOP-PHI-GENERIC-PHYSICAL-DEMAND-D0

## Six-line brief

```text
Decision: stop the generic Ready Loop bridge until its logical Recipe has one canonical physical-demand handoff.
Source authority + canonical issuer: CallableSemanticLoweringState and the claimed GenericLoopV1 Recipe own BindingRef/role/site; CanonicalSsaFunctionSessionV2 owns ValueId/CFG/PHI/seal issuance.
Non-authority: generic_loop_composer.rs, CorePhiInfo, variable_map, composer-local phi_bindings/carrier_step_phis, latest values, and the raw loop root scope.
Fail-fast boundary: before any Builder effect, reject a missing or mismatched operation/effect/continuation relation, owner, loop site, binding, placement, or result class; never fall back to the old composer after this boundary.
Smallest next slice: audit whether the existing GenericLoopV1 Recipe can be projected directly into PreparedLoopOperationProgramV1 and the canonical physicalizer without re-reading AST or issuing another semantic product.
Non-claims: no second PHI issuer, no plan-level adapter, no raw-port session field, no local-completion handoff, no backend/OBJ/EXE, and no R7 retirement.
```

## Why I0 cannot open the consumer yet

The current callable Ready consumer receives a claimed
`CallableGenericLoopV1SemanticRecipeV1`, then calls
`compose_source_generic_loop_v1_recipe_with_port`. That composer allocates a
CorePlan skeleton and name-keyed carrier/PHI state before the existing
`CanonicalSsaFunctionSessionV2` can own the physical blocks and seals. Passing
the callable ledger into the composer or wrapping its maps would leave two
physical value authorities. Calling the old composer from a session wrapper
would therefore violate the accepted PHI value-flow contract.

The common physicalizer already consumes a complete
`PreparedLoopOperationProgramV1` and borrows the canonical session's CFG,
identity, and `PhiTxn`. The missing fact is whether the current generic
Recipe's `PlanBuildOutcome`/`CanonicalLoopFacts` contains that complete
operation/effect/continuation product without another AST walk or semantic
reclassification. This is a physical-demand boundary question, not a reason
to add a second MirBuilder.

## Ownership and decision test

The audit must trace one exact chain:

```text
selected callable entry
  -> GenericLoopV1 source Facts/Recipe claim
  -> existing operation/effect/continuation product, if present
  -> PreparedLoopOperationProgramV1
  -> CanonicalSsaFunctionSessionV2 physicalizer
```

Choose **Reuse** only when every operation carries its existing source-bound
item/binding/site relation, the continuation identifies the same loop and
After binding, and the prepared program can be built before Builder effects.
Choose **Design a compiler-side projection** only when the projection consumes
an existing claimed Recipe and publishes no new source meaning, selector,
BindingRef, or physical ID. Otherwise remain `NoSafeSlice` and do not add an
adapter or fixture.

The old composer remains a compatibility consumer while this row is stopped.
It cannot be described as canonical physical evidence and cannot be used to
close the session-entry acceptance row.

## Ordered tasks

1. Inventory the exact fields exposed by `CallableGenericLoopV1SemanticRecipeV1`,
   `PlanBuildOutcome`, and the common operation/effect products. Record which
   fields are source facts, Recipe rows, or physical-demand rows.
2. Prove or refute a builder-free projection to
   `PreparedLoopOperationProgramV1`. The proof must preserve Recipe order,
   operation placement, BindingRef identity, continuation/After binding, and
   owner/frame/site equality without AST reparse or name lookup.
3. If Reuse is proven, add one private compiler-side handoff and a named
   reject for every missing relation. If not, record `NoSafeSlice` with the
   missing owner and reopen a bounded design card; do not implement a partial
   projection.
4. Only after the handoff is accepted, reopen I0's session consumer: one
   function-scoped `CanonicalSsaFunctionSessionV2`, canonical header/body/
   backedge/After values, and explicit discard on failure.
5. Add a valid source fixture without manual ledger injection and then the
   zero/one/multiple iteration plus one-relation mutation matrix. These are
   evidence for the session consumer, not permission to use the old Composer.

## Acceptance and delete set

This row closes only when the logical-to-physical handoff has one named owner,
one consumer, a pre-effect fail-fast boundary, and a proven exclusive delete
set. The selected old composer path may be deleted only after the new consumer
is production-connected and the generic Loop acceptance matrix passes. No
CorePhiInfo, common PlanLowerer, dynamic/non-callable Loop route, or
compatibility reader is in this delete set.

