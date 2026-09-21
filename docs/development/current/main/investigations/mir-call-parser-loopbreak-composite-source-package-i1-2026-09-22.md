---
Status: closeout__2026-09-22__CompositePackageRecipeMappingGreen
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-I1
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-body-role-i0-2026-09-22.md
Implementation permission: true for package retention and source-bound Recipe construction only
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PHYSICAL-I2
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

## Decision — 2026-09-22

Use an explicit tagged candidate at the existing package boundary. Preserve
the current direct `Candidate(VerifiedCallableLoopBreakSourceFactsV1)` loan and
add a sibling `CompositeCandidate` loan carrying the I0 projection plus its
source-bound Recipe. The package issuer remains the sole row classifier; no
shared envelope or name-based reclassification is added. The Recipe producer
is an extension of the existing `loop_break_builder` owner, and it consumes
the role tree exactly once while retaining the exact source AST bodies needed
by `RecipeBodies`/`StmtRef`.

This decision authorizes only the package-row disposition, one-shot composite
take/finish, and source-bound Recipe construction/verification. The physical
adapter, production caller switch, and old-edge deletion remain I2 and later.

## Scheduler handoff — 2026-09-22

The warning cohort is intentionally paused at I147: the unused-import tail is
17 and the remaining `dead_code` count is owner debt, not a reason to keep the
semantic lane waiting. No warning suppression, test deletion, or bulk
`dead_code` cleanup belongs in this I1 slice. The next executable work is the
package/Recipe mapping below; after it, I2 owns the physical adapter and only
then may a later row consider caller switch and old-edge deletion.

## Ordered task rows

1. **I1 package/Recipe mapping** — add the tagged `Direct`/`Composite` row to
   the existing package issuer, retain the composite projection exactly once,
   build and verify the source-bound Recipe, and prove residual/duplicate/
   second-take rejection while keeping direct rows green.
2. **I2 physical handoff** — consume the verified composite Recipe through the
   existing LoopBreak physical owner; this row must identify the exact source
   port and selected target relation before any Builder effect.
3. **I3 production cutover** — switch the selected parser caller, run the
   source-to-MIR acceptance path, and delete the named old edge only after
   caller-zero and guard evidence are present.

Rows 2 and 3 are planning entries only until I1 acceptance is recorded.

## I1 evidence — 2026-09-22

The tagged package/Recipe slice is complete at its bounded boundary. The
parser program fixture retains one `CompositeCandidate`, the package owner
accepts the exact owner row once, the composite Facts consume the exact loop
site once, and `finish_empty` succeeds with no residual candidate. The direct
candidate and typed-absence rows remain green in the same focused run.

Evidence:

* `CARGO_BUILD_JOBS=4 cargo test --profile quick --lib normal_callable_semantic_package::loop_break_source_tests -- --nocapture`
  — 8 passed, 0 failed, including the parser composite package test;
* `cargo check --profile quick --lib` — exit 0; 1,674 warnings, matching the
  existing warning baseline after the I147 cohort. No warning suppression,
  test deletion, or dead-code claim was added by this slice;
* `cargo fmt --all -- --check` and `bash tools/checks/current_state_pointer_guard.sh`
  — green;
* all new source files remain below the 760-line design threshold (the
  existing LoopBreak Facts owner is 745 lines).

This closes only package retention and source-bound Recipe construction. I2
still needs a named physical source-port/target relation before allocation;
there is no source-to-MIR publication, production caller switch, fallback
change, or old-edge deletion claim.

## Acceptance and non-claims

I1 is accepted only when the package owner can retain one composite row,
consume it once for its exact owner/site, verify the complete role-to-Recipe
mapping, and finish with no residual candidate. Direct package tests must remain
green. This still does not authorize the physical adapter, production caller
switch, `GenericLoopV1NotSelected` retirement, or old-edge deletion.
