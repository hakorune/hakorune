Status: active__Implementation__GenericG0ProductionTerminal__2026-09-11
Task: LOOP-G0-PRODUCTION-TERMINAL-I1
Date: 2026-09-11
Priority: connect one Generic G0 physical lowerer to the existing Single completion/publication spine
Parent: mirbuilder-loop-g0-canonical-issuer-i0-2026-09-11
NextCard: LOOP-G0-SOURCE-TO-EXE-PUBLICATION-I2
---

# Generic G0 production terminal I1

## Six-line brief

```text
Decision: consume the co-sealed GenericG0 plan through one production physical lowerer and the existing Single collect/complete/drain spine.
Source authority + canonical issuer: CanonicalGenericG0PlanV1 and its source parent; the I1 lowerer may borrow existing Generic physical admission products but may not reissue source facts.
Non-authority: generic_g0_physical_emitter_session test helper, route_loop, MIR/name/arity inference, direct publication, retry, and legacy Generic routes.
Fail-fast boundary: admission, shell, entry/control mapping, draft seal, and collector compatibility reject before any externally published module state.
Smallest next slice: name the one production lowerer input and wire lower -> collect -> complete -> drain with unpublished failure evidence.
Non-claims: no all-family Loop cutover, backend parity, source-to-exe success, old-edge deletion, or whole-MIRBuilder completion.
```

## Authorized implementation cells

1. The existing `CanonicalGenericG0PlanV1`/source-parent handoff and one
   Generic-only physical admission owner. Reuse existing admission and common
   segment/layout owners; do not create a second source parent or physical
   identity issuer.
2. One production sibling under
   `src/mir/builder/resolved_lowering/loop_recipe_physicalizer/` that consumes
   the co-sealed source products and returns the existing `MirFunction` draft
   shape. The test-only `generic_g0_physical_emitter_session` is not promoted.
3. The existing package `consume`, Single collector, completion, manifest
   drain, finalization, postprocess, external commit, and `publish_once`
   callers only. No new token, continuation, manifest, or publication route.
4. Focused positive/negative tests and one reusable guard proving the
   no-test-session/no-route-loop/no-second-owner boundary, plus this README and
   the canonical Loop reference.

If the lowerer cannot consume the co-sealed source products without
reconstructing an effect, ABI, entry lane, or physical identity, return to a
design stop and name the missing issuer; do not add a default or fallback.

Existing separate follow-ups remain open and are not silently folded into I1:
the LocalSSA failure cache, measurement-off compile cost, and view re-scan;
the G0 production connection itself is also not claimed by I0.

## Fixed terminal mapping

```text
CanonicalGenericG0PlanV1
  -> one Generic physical admission/lowerer
  -> LoweredCanonicalPlanV1::Single
  -> existing collect -> complete -> prepare_drain -> drain
  -> finalization -> postprocess -> external commit -> publish_once
```

The next card is source-to-exe acceptance only after this lowerer has both
positive draft/collection evidence and zero-publication rejection evidence.
