# Generic G0 Recipe S4 I0/R0 implementation task

Status: `landed caller-zero implementation 2026-08-07; production remains closed`

Parent design: `generic-g0-recipe-s4-design-task-2026-08-07.md`.

## Scope

Implemented one `cfg(test)` caller-zero Generic G0 Recipe producer. It
consumes `VerifiedGenericRecipeDemandG0` exactly once and emits the portable
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

## Landed evidence

- natural G0 demand is consumed once and the deterministic product seals;
- one deterministic golden Recipe verifies with three carrier rows and the
  exact ten-row source/effect matrix;
- source forest binding and Generic provenance are issued once;
- JoinSig and After are common-owned and requested exactly once;
- post-loop read, `L0.After/b1`, owner/frame, and exact return ABI co-seal;
- P0 completion/DraftSeal remains untouched;
- common contract tests cover duplicate/missing/foreign relation, wrong anchor,
  stale provenance, and source reissue; After/ABI pairing has typed rejects;
- source modules remain below 800 lines and no production caller appears;
- the implementation commit updates the parent task receipt,
  `docs/reference/mir/generic-loop-stage-matrix.md`, Generic source-to-Recipe
  SSOT, affected module READMEs, and current mirrors together;
- focused tests, shared loop-family guard, pointer guard, and `git diff --check`
  are green before commit/push.

Focused result: `cargo test --lib generic_g0 --features plugins` passed with
42 tests, including the two S4 producer tests. The producer remains test-only;
it does not open selection, Builder/MIR, physical lowering, completion,
retry/fallback, or legacy deletion.

## Explicit non-goals

This row does not claim a physical recipe, executable Return completion,
MIR/PHI parity, M8 all-row coverage, selfhost parity, production cutover, or
legacy retirement. Those remain separate ordered rows in the parent SSOT.
