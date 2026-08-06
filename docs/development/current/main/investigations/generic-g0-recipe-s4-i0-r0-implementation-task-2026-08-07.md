# Generic G0 Recipe S4 I0/R0 implementation task

Status: `next implementation row; design accepted 2026-08-07; implementation not started`

Parent design: `generic-g0-recipe-s4-design-task-2026-08-07.md`.

## Scope

Implement one `cfg(test)` caller-zero Generic G0 Recipe producer. It must
consume `VerifiedGenericRecipeDemandG0` exactly once and emit the portable
`VerifiedGenericRecipeProductG0` described by the parent design. This row does
not open production selection, Builder/MIR, physical lowering, completion,
retry/fallback, or legacy deletion.

## Required sequence

```text
demand.into_parts()
  -> bind resolved source forest once
  -> private deterministic G0 Recipe draft/key map
  -> LoopRecipeVerifierV1::verify / source-bound verification
  -> LoopJoinSigElaboratorV1::elaborate once
  -> require_after_binding(L0, b1, I64) once
  -> exact source/effect relations
  -> issue_source_bound_core_v1
  -> Generic After/tail/ABI envelope
  -> VerifiedGenericRecipeProductG0
```

The producer must use no AST/source-view/name lookup, `RecipeBody`/
`RecipeBlock`, route ID, legacy scheduler, `ValueId`, `BasicBlockId`, PHI,
Builder, MIR, retry, fallback, or Generic-specific physicalizer.

## Acceptance

- natural G0 demand is consumed once and stale/foreign demand rejects;
- one deterministic golden Recipe verifies and round-trips;
- three carrier rows and exact ten-row source/effect matrix are asserted;
- source forest binding and Generic provenance are issued once;
- JoinSig and After are common-owned and requested exactly once;
- post-loop read, `L0.After/b1`, owner/frame, and exact return ABI co-seal;
- P0 completion/DraftSeal remains untouched;
- negative fixtures cover duplicate/missing/foreign relation, wrong anchor,
  wrong After binding, wrong ABI, stale provenance, and source reissue;
- source modules remain below 800 lines and no production caller appears;
- the implementation commit updates the parent task receipt,
  `docs/reference/mir/generic-loop-stage-matrix.md`, Generic source-to-Recipe
  SSOT, affected module READMEs, and current mirrors together;
- focused tests, shared loop-family guard, pointer guard, and `git diff --check`
  are green before commit/push.

## Explicit non-goals

This row does not claim a physical recipe, executable Return completion,
MIR/PHI parity, M8 all-row coverage, selfhost parity, production cutover, or
legacy retirement. Those remain separate ordered rows in the parent SSOT.
