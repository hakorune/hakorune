# Located GenericLoopV1 representation

This module seals one exact PATH0 `Loop` carrier together with the canonical
`GenericLoopV1ExtractionV1` produced from that same carrier.

It is analysis-only. It must not allocate MIR skeletons, mutate a `MirBuilder`,
claim callable-result ledger rows, reconstruct source identity from cloned AST,
or reclassify step placement after extraction.

The first admitted profile is `NumericProgression + StepPlacement::Last` with
one of two lowering representations:

- `DirectRecipeOnly`: exact direct source prefix plus exact cleanup carrier.
- `ExitAllowedRecipe`: an existing recipe tree sealed in ordinal lockstep with
  exact PATH0 carriers. A recipe `Stmt` whose exact source is an `If` is sealed
  once through the existing NoExit recipe owner as `StmtWrappedJoinIf`.

The verified products are deliberately non-`Clone`. Production lowering does
not consume them until the later T0 row.
