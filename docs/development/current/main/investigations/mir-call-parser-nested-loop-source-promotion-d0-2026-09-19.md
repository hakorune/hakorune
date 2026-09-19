---
Status: selected__design_stop__2026-09-19
Task: MIR-CALL-PARSER-NESTED-LOOP-SOURCE-PROMOTION-D0
Date: 2026-09-19
Parent: mir-call-static-compatibility-catalog-target-i0-2026-09-19.md
ProductionCaller: selected normal MIR/static-receiver route only
Implementation permission: false; this card fixes the source-product and route boundary only
Classification: BoxCount; one bounded source-loop admission profile
---

# Parser nested-loop source promotion D0

## Six-line brief

```text
Decision: decide whether the existing source-aware GenericLoop Facts/Recipe/
JoinSig/physical chain can admit one parser nested/exit-driven profile as one
co-sealed product; keep the tuple at a typed stop until that decision is accepted.
Source authority + canonical issuer: parser-branded merged source plus the
resolver-issued loop forest and resolved-exit records, consumed by the existing
CallableGenericLoopSourceFactsIssuerV1 chain.
Non-authority: parser line numbers, names/arity, AST/MIR rescans, LoopBreak's
legacy Builder route, VM/compatibility retry, and a parser-specific issuer.
Fail-fast boundary: missing or foreign root/forest/exit/target identity,
overlap with an existing route, or an unconsumed nested transfer rejects before
catalog, argument effects, or physical lowering.
Smallest next slice: design one co-sealed forest-plus-exit source profile,
route precedence, physical consumer contract, and finite negative matrix.
Non-claims: no implementation, Cataloged/Selected publication, static consume,
production switch, fallback removal, VM promotion, or legacy retirement.
```

## Exact finite profile

The only candidate is the merged source row already selected by the parent
static card:

```text
caller       = ParserProgramBox.parse/2
target       = ParserStringUtilsBox.starts_with/3
target site  = parser_program_box.hako:102 (source identity, not line lookup)
root loop    = :81 loop(cont_prog == 1)
child loops  = :131 static_semis == 1; :182 loop(true) semicolon scan
root exits   = :85/:95 break; :104/:109/:127/:142/:161/:165/:194 returns;
               the post-loop :218 return is outside this loop and remains a
               separate terminal
child exits  = :186 continue and :188 break in the semicolon-scan child;
               the static-semicolon child has a condition-only exit
result       = the parent's ExactI64/[1] static-result tuple
```

The line numbers identify the reviewed counterexample only. The product must
bind the resolver's `SourceStmtSiteV1`/`SourceExprSiteV1`, parent indices, loop
frame keys, and `ResolvedExitRecordV1` rows from the same invocation. A missing
member, omitted nested loop, detached exit, duplicate exit, or foreign owner
is a typed rejection; it is never rounded to a direct condition/body pair.

## Existing authority candidate

Candidate A is the only authority allowed into this design:

```text
RawInvocationSourceContextV1
  -> CallableGenericLoopSourceFactsIssuerV1
  -> CallableGenericLoopV1SemanticRecipeV1
  -> RecipeFirstRouteSelectionV1
  -> existing GenericLoop JoinSig/physical adapter
  -> existing completion/DraftSeal and static publication consumer
```

The extension, if accepted, must add the loop forest, ordered body paths,
resolved exits, and target/source relation to this existing move-only product
and consume them in the same Recipe/JoinSig/physical transaction. The source
issuer remains the sole issuer; no sidecar forest observer may be paired later.

`LoopBreakRecipe` is not a candidate: its direct three-statement terminal and
legacy `MirBuilder` route do not carry the source handoff. A new parser route,
parser-specific Recipe issuer, AST rescan, or compatibility retry is also
rejected by this D0.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Forest/exit co-seal | One existing source Facts owner binds root, both child loops, parent indices, frame keys, all resolver exits, and the `starts_with` source site from the same invocation. |
| 2 | Generic route precedence | The existing registry has one unambiguous source-backed route for this profile; ordinary Generic, LoopBreak, nested omission, and overlapping raw schedules keep typed rejects. |
| 3 | Recipe/JoinSig contract | The existing Recipe and JoinSig expose ordered nested segments and exit transfers without reconstructing meaning from AST/MIR or names. |
| 4 | Physical consumer contract | The existing physical adapter can consume every co-sealed segment/exit and finish or discard the whole session; no partial static argument effects are allowed. |
| 5 | Negative ownership matrix | Foreign root/brand, parent drift, missing/duplicate exit, wrong target, omitted child, extra loop, wrong condition, and route overlap reject before catalog/effects. |
| 6 | Parent handoff | Only after tasks 1–5 are accepted may the parent static card observe one Cataloged row and later design its Selected/retirement evidence. |

## Stop boundary and acceptance

This remains `design_stop`. Do not edit Rust/Hako, add fixtures, issue a new
semantic receipt, or move the static tuple to Cataloged until the existing
authority can be named for every task above. If Candidate A cannot carry the
complete forest and exit set without a second authority, retain
`GenericLoopV1NotSelected` and close this D0 as `NoSafeSlice` with the missing
field/consumer named.

The positive acceptance shape is deliberately one source invocation: the
merged parser route reaches the existing Generic source product with all
three loop members and all exits co-sealed, then the parent static tuple may
continue to Cataloged. It does not claim physical lowering, source-to-exe,
compatibility retirement, or any other parser callsite.

## Evidence used

- `mir-call-static-compatibility-catalog-target-i0-2026-09-19.md` — named
  `GenericLoopV1NotSelected` terminal and selected static tuple.
- `mir-call-parser-loop-forest-exit-coseal-d0-2026-09-19.md` — resolver forest
  and exit evidence plus the missing callable co-seal.
- `mirbuilder-callable-loop-ready-generic-loop-v1-recipe-authority-d0-2026-08-22.md`
  — existing GenericLoop source Facts/Recipe/physical boundary.
- `lang/src/compiler/parser/program/parser_program_box.hako:81-189` — finite
  nested/exit-driven counterexample shape.
