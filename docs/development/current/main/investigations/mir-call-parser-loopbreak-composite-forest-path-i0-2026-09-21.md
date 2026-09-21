---
Status: closeout__2026-09-21__IfBranchLoopSourcePathExtension
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-PATH-I0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-PATH-I0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-composite-forest-boundary-d0-2026-09-21.md
Implementation permission: true for one existing resolver/path owner extension; no package/Recipe/physical/cutover change
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-D0
---

# Parser composite LoopBreak forest path I0

## Six-line brief

```text
Decision: extend the existing resolver forest and portable loop source-path grammar to retain IfThen/IfElse descendants for the finite parser composite.
Source authority + canonical issuer: VerifiedResolvedFunctionV1::resolved_loop_source_forest and bind_resolved_loop_source_forest_v1; existing IfSourcePathStepV1 naming is vocabulary guidance only.
Non-authority: AST/name/line rescans, a second forest issuer, LoopCond reclassification, LoopRouteContext, Recipe keys/items, package admission, physical IDs, fallback, retry, VM/backend.
Fail-fast boundary: exact parent prefix, branch index, no skipped intermediate loop, no orphan body-root, and resolver-issued member/exit identity before a portable path is issued.
Smallest next slice: add typed IfThenItem/IfElseItem path variants, permit those segments in forest ancestry and source-binding verification, and prove the parser root plus synthetic positive/negative paths.
Non-claims: no LoopBreak Recipe mapping, package retention, source-to-MIR publication, production switch, old-edge deletion, backend parity, or warning cleanup.
```

## Finite source boundary

The selected parser root is `parser_program_box.hako:81` with nested members:

```text
root      Body(22)
child one Body(22) -> LoopBody(8) -> IfThen(3)   (:131)
child two Body(22) -> LoopBody(9) -> IfThen(8)   (:182)
```

The resolver already owns the loop regions, member order, parent relation, and
exit ledger. Only the portable path grammar rejects the conditional ancestors.
The path extension must remain lossless for `IfElse` as well, since the
resolver source vocabulary and existing if contract already distinguish both
branches.

## Implementation tasks and completion evidence

| order | task | completion condition |
| --- | --- | --- |
| 1 | Extend `LoopSourcePathStepV1` | add typed `IfThenItem`/`IfElseItem` variants with the existing serde/deny-unknown contract |
| 2 | Extend resolver forest ancestry | `forest_parent_index` accepts only branch-body items in the exact descendant position and preserves parent indices; orphan/skip/foreign cases remain named rejects |
| 3 | Extend portable adapter and verifier | `portable_path_v1` and `LoopRecipeSourceClaimVerifierV1` accept branch steps only after the required `LoopBodyItem`, with no body/loop re-entry or skipped parent |
| 4 | Focused guards | synthetic nested-if loop positive path, `IfElse` positive path, skipped-loop/branch-root/foreign negative paths, and existing path tests remain green |
| 5 | Real parser source check | the I2 merged parser guard no longer reports root `ForestLookup` or branch `UnsupportedAncestor`; it issues one forest with the root and both finite child members |

No package or Recipe consumer may be changed in this card. If any downstream
consumer requires a new semantic relation beyond this path grammar, stop and
open a new design card instead of widening I0.

## Implementation and evidence — 2026-09-21

The existing resolver/path owner now retains conditional descendants without a
second issuer. `LoopSourcePathStepV1` has typed `IfThenItem` and `IfElseItem`
variants; resolver forest ancestry accepts those segments only after the
required loop-body entry; the portable adapter preserves them; and the source
binding verifier accepts them only as branch suffixes after the exact parent
prefix. Body re-entry, skipped intermediate loops, orphan roots, and foreign
owners remain named rejects.

Focused evidence:

```text
CARGO_BUILD_JOBS=4 cargo test --profile quick --lib loop_structural_facts
28 passed, 0 failed; warning baseline 560

CARGO_BUILD_JOBS=4 cargo test --profile quick --lib loop_break_composite_source_projection
3 passed, 0 failed; warning baseline 560
```

The first suite covers the synthetic `IfThen`/`IfElse` forest and the
resolver-to-adapter binding guards. The second suite covers the direct
composite product, foreign-root rejection, and the merged
`parser_program_box.hako` source. The real parser check now issues a forest
with the root plus both conditional child members and no
`Forest(ForestLookup)` or `Forest(ForestBinding(...))` reject. The full
`loop_recipe_contract` filter was also compiled; its new branch-path test is
green, while one pre-existing unrelated
`source_bound_core_rejects_derived_carrier_and_duplicate_effect_mismatch`
failure remains known baseline debt outside this card's files.

This closes the source-path capability only. Package retention, composite
Recipe mapping, physical publication, production cutover, caller-zero proof,
and old-edge deletion remain unclaimed and must be designed in the successor
package row.
