---
Status: design_stop__2026-09-22__CompositePackageRecipeMapping
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-I1
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-body-role-i0-2026-09-22.md
Implementation permission: false; accept the package/Recipe relation before editing the package owner
NextCard: owner-decision__composite_package_recipe_mapping
---

# Parser composite LoopBreak package and Recipe mapping I1

## Six-line brief

```text
Decision: extend the existing LoopBreak package row with an explicit
  Direct|Composite candidate and map the composite body-role tree once into
  the existing Recipe vocabulary.
Source authority + canonical issuer: ResolvedFunctionLoweringInputV1,
  resolver forest/exit ledger, and issue_loop_break_composite_source_projection_v1
  as the composite source issuer; issue_loop_break_source_package_v1 remains
  the package-row owner; build_loop_break_source_recipe remains the Recipe owner.
Non-authority: names/line numbers, AST rescans, LoopRouteContext, generic
  retry/fallback, VM routes, synthetic meaning, independent StmtRef/BodyId
  reconstruction, or MIR/physical IDs.
Fail-fast boundary: exact owner/origin/source-kind/root relation, complete
  forest member coverage, one-to-one role-to-RecipeItem mapping, branch/exit
  relation, exact source-call/target relation, and duplicate/missing/foreign/
  out-of-root/unsupported/residual/second-take rows reject before effects.
Smallest next slice: retain the composite projection in the existing one-shot
  package loan and build/verify one source-bound Recipe using Stmt, IfV2,
  LoopV0, Exit, and RecipeBodies while preserving direct-shape behavior.
Non-claims: no physical lowering/publication, source-to-MIR acceptance,
  production caller switch, old-edge deletion, compatibility fallback,
  VM/backend work, or warning cleanup.
```

## Finite census boundary

`normal_callable_semantic_package::loop_break_source::issue_loop_break_source_package_v1`
is the first package owner and `LoopBreakSourcePackageLoanV1::finish_empty` / its
one-shot owner take is the terminal. The selected source family is the parser
composite LoopBreak roots already admitted by the resolver forest; direct
three-statement LoopBreak rows, non-candidate typed absence, other backends,
and physical consumers are excluded from this I1 design boundary.

## Existing owners and relation

The current package loan owns only `VerifiedCallableLoopBreakSourceFactsV1`.
The direct source Facts issuer still calls
`issue_loop_break_source_projection_v1`, and the direct Recipe constructor
`build_loop_break_source_recipe` deliberately requires the three-statement
shape. I1 must add an explicit composite disposition in these existing owners;
relaxing the direct predicate or silently treating a composite root as direct
would merge two meanings and reopen the old generic observer.

The composite candidate must retain the I0
`VerifiedLoopBreakCompositeSourceProjectionV1` together with the existing
planner/terminality evidence only after the source-to-Recipe relation is
complete. The Recipe producer then consumes the role tree exactly once:

| role | existing Recipe item | required relation |
| --- | --- | --- |
| `Statement` | `RecipeItem::Stmt` | exact source statement site |
| `If` | `RecipeItem::IfV2` | condition site, branch bodies, resolver branch relation |
| `Loop` | `RecipeItem::LoopV0` | forest member index, parent index, frame and condition |
| `Exit` | `RecipeItem::Exit` | resolver exit record and enclosing branch/target |

No new Recipe key, JoinSig key, physical ID, or target authority is issued by
this row. Existing `RecipeBodies`/verification must reject a missing child,
duplicate site, foreign owner, residual role, or a second package take before
Builder effects. The I0 method-call inventory is root-contained evidence only;
selected target and exact argument requirement must be joined from the
existing source-call target relation rather than inferred from the call name.

## Required design decision before implementation

Choose whether the package row is represented as a tagged candidate enum
(`Direct`/`Composite`) or as a shared candidate envelope with two source
projection variants. The choice must preserve direct callers and the existing
one-shot `take_candidate_for_site` terminal, and must name the exact consumer
that maps the composite Recipe. No implementation is permitted until that
choice and the role-to-Recipe coverage tuple are accepted.

## Acceptance and non-claims

I1 is accepted only when the package owner can retain one composite row,
consume it once for its exact owner/site, verify the complete role-to-Recipe
mapping, and finish with no residual candidate. Direct package tests must remain
green. This still does not authorize the physical adapter, production caller
switch, `GenericLoopV1NotSelected` retirement, or old-edge deletion.

