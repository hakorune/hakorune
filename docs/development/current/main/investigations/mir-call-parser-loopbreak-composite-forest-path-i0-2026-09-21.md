---
Status: fast__2026-09-21__IfBranchLoopSourcePathExtension
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-PATH-I0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-FOREST-PATH-I0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-composite-forest-boundary-d0-2026-09-21.md
Implementation permission: true for one existing resolver/path owner extension; no package/Recipe/physical/cutover change
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PACKAGE-I3
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
