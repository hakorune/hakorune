---
Status: closed__2026-09-21__DirectSourceBodyInventoryGuard
Task: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I0
Date: 2026-09-21
Parent: mir-call-parser-source-to-mir-package-acceptance-window-d0-2026-09-20.md
Implementation permission: true for one source-body role inventory and its existing-owner guard; no Recipe/physical/cutover change
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-PRODUCT-I1
---

# Parser composite LoopBreak source product I0

## Four-block execution brief

```text
Decision: extend the existing LoopBreak source projector with one exact,
  resolver-branded body-role inventory for the selected direct source owner;
  keep the parser composite root as the next bounded source-product row.
Source authority + canonical issuer: the same invocation's
  ResolvedFunctionLoweringInputV1 source view, resolver forest/exit ledger,
  and CallableGenericLoopSourceFactsIssuerV1.
Non-authority: RecipeItem/StmtRef/BodyId, AST/name/line rescan, LoopRouteContext,
  child-route inference, physical IDs, fallback, retry, or package filtering.
Fail-fast boundary: foreign owner, missing/duplicate/out-of-root source site,
  unsupported body child, missing exit record, or transfer mismatch rejects
  before any Builder effect.
Smallest next slice: issue and validate the ordered source body-role inventory
  on the existing direct projection, while preserving its three-statement
  candidate behavior. The parser root `:81` with nested `:131`/`:182` remains
  an explicit follow-up because its composite consumer is not yet selected.
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

The direct three-statement shape continues to issue the existing candidate
unchanged and now retains the inventory. Composite bodies still do not issue a
candidate in this row; they must not be silently downgraded to an empty
candidate or routed through LoopCond.

## I0 closeout evidence

`cargo test --profile quick --lib loop_break_source_projection -- --nocapture`
completed with 6/6 tests passing on 2026-09-21. The direct positive fixture
checks owner/root identity, four ordered body statements, and no nested-loop
rows. Existing foreign-owner, scope-box, explicit-else, forest, and duplicate
site guards remain green. The quick build emitted the repository's existing
warning baseline; this row made no warning policy claim.

## Focused acceptance

| case | required result |
| --- | --- |
| direct LoopBreak root | ordered inventory preserves owner/root and exact body sites |
| foreign or duplicated site | named reject before Builder effects |
| unsupported child body | typed reject; no fallback or route reclassification |
| existing direct three-statement fixture | prior source projection and physical matrix remain unchanged |

The parser composite root `:81`/`:131`/`:182`, its full exit ledger, and its
Recipe consumer are intentionally carried to `I1`; this I0 does not claim
package acceptance or composite production reachability.

Closeout requires focused positive/negative/guard evidence, the module README
or source-contract note when the public boundary changes, and a clean pointer.
The next Recipe row may consume this inventory only after this row's one-shot
source ownership is proven.
