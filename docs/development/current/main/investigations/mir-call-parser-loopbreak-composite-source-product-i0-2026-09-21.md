---
Status: fast__2026-09-21__ParserLoopBreakCompositeSourceProduct
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I0
Date: 2026-09-21
Parent: mir-call-parser-source-to-mir-package-acceptance-window-d0-2026-09-20.md
Implementation permission: true for one source-body role inventory and its existing-owner guard; no Recipe/physical/cutover change
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-RECIPE-I0
---

# Parser composite LoopBreak source product I0

## Four-block execution brief

```text
Decision: extend the existing LoopBreak source projector with one exact,
  resolver-branded body-role inventory for the finite parser composite root.
Source authority + canonical issuer: the same invocation's
  ResolvedFunctionLoweringInputV1 source view, resolver forest/exit ledger,
  and CallableGenericLoopSourceFactsIssuerV1.
Non-authority: RecipeItem/StmtRef/BodyId, AST/name/line rescan, LoopRouteContext,
  child-route inference, physical IDs, fallback, retry, or package filtering.
Fail-fast boundary: foreign owner, missing/duplicate/out-of-root source site,
  unsupported body child, missing exit record, or transfer mismatch rejects
  before any Builder effect.
Smallest next slice: issue and validate the ordered source body-role inventory
  for parser root `:81`, including nested `:131`/`:182` bodies and exact exit
  sites, while keeping the existing direct three-statement candidate behavior.
Non-claims: no composite Recipe, physical lowering, package success, production
  switch, old-edge deletion, backend parity, or warning cleanup.
```

## Owner and relation contract

The projector remains the sole source-site issuer for the existing LoopBreak
Facts owner. The new inventory carries only exact `SourceStmtSiteV1`/
`SourceExprSiteV1` sites, parent-body membership, child-loop membership, and
paired `ResolvedExitRecordV1` rows. It contains no Recipe key, selector cursor,
MIR identity, or route choice. The existing forest projection remains the
parentage authority; the planner remains the child route authority.

The direct three-statement shape must continue to issue the existing candidate
unchanged. Composite bodies receive the inventory as a source-contract product
for the next Recipe slice; they must not be silently downgraded to an empty
candidate or routed through LoopCond.

## Focused acceptance

| case | required result |
| --- | --- |
| parser root `:81` with children `:131` and `:182` | ordered inventory preserves exact parentage and source sites |
| root breaks `:85`/`:95`, child continue/break, and returns | each transfer is paired with its resolver exit record |
| foreign or duplicated site | named reject before Builder effects |
| unsupported child body or missing transfer | typed reject; no fallback or route reclassification |
| existing direct three-statement fixture | prior source projection and physical matrix remain unchanged |

Closeout requires focused positive/negative/guard evidence, the module README
or source-contract note when the public boundary changes, and a clean pointer.
The next Recipe row may consume this inventory only after this row's one-shot
source ownership is proven.
