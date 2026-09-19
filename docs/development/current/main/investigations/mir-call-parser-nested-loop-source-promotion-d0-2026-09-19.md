---
Status: accepted__design__implementation_child_selected__2026-09-19
Task: MIR-CALL-PARSER-NESTED-LOOP-SOURCE-PROMOTION-D0
Date: 2026-09-19
Parent: mir-call-static-compatibility-catalog-target-i0-2026-09-19.md
ProductionCaller: selected normal MIR/static-receiver route only
Implementation permission: false; design accepted and handed to the bounded source-handoff I0
Classification: BoxCount; one bounded source-loop admission profile
---

# Parser nested-loop source promotion D0

## Six-line brief

```text
Decision: decide whether the existing LoopCondBreakContinue Facts/Recipe/
JoinSig/physical chain can admit one parser nested/exit-driven profile as one
co-sealed product; keep the tuple at a typed stop until that decision is accepted.
Source authority + canonical issuer: parser-branded merged source plus the
resolver-issued loop forest and resolved-exit records, consumed by one
source-aware extension of the existing LoopCondBreakContinue Facts/Recipe owner.
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
  -> existing LoopCondBreakContinue Facts/Recipe owner
  -> one source-aware co-sealed handoff under that owner
  -> RecipeFirstRouteSelectionV1
  -> existing LoopCondBreakContinue JoinSig/physical pipeline
  -> existing completion/DraftSeal and static publication consumer
```

The parser outer condition is a state-machine condition, not a progression
loop: it has no GenericLoop `loop_var`/increment pair. The existing GenericLoop
source adapter therefore cannot be the authority for this profile; its nested
source path explicitly rejects `UnsupportedFirstCohort`. The existing
LoopCondBreakContinue facts can represent the reviewed shape through its
`ProgramBlock` and `NestedLoopDepth1` recipe items. The extension, if accepted,
must add the loop forest, ordered body paths, resolved exits, and target/source
relation to that owner and consume them in one Recipe/JoinSig/physical
transaction. The source issuer remains the sole issuer; no sidecar forest
observer may be paired later.

`LoopBreakRecipe` is not a candidate: its direct three-statement terminal and
legacy `MirBuilder` route do not carry the source handoff. A new parser route,
parser-specific Recipe issuer, AST rescan, or compatibility retry is also
rejected by this D0.

## Decision — 2026-09-19

Candidate A is accepted as a same-owner extension of the existing
`LoopCondBreakContinue` Facts/Recipe/physical pipeline. The source product will
carry the resolver forest, ordered source paths, resolved exits, and selected
static target relation together. `RecipeFirstRouteSelectionV1` remains the
route authority; the source handoff adds a typed LoopCond selection token
instead of re-running the planner or manufacturing a Generic route.

The existing GenericLoop owner is explicitly outside this decision because the
parser root has no progression variable/increment pair and the source context
rejects nested lowering. The existing LoopBreak owner remains compatibility
only because its direct terminal requires the legacy `MirBuilder` route. This
is one BoxCount extension of the LoopCond source contract, not a second route
or a parser-specific semantic family.

## Ordered design tasks

| Order | Task | Completion condition |
| --- | --- | --- |
| 1 | Forest/exit co-seal | One existing source Facts owner binds root, both child loops, parent indices, frame keys, all resolver exits, and the `starts_with` source site from the same invocation. |
| 2 | LoopCond route precedence | The existing registry has one unambiguous source-backed LoopCondBreakContinue route for this profile; ordinary Generic, LoopBreak, nested omission, and overlapping raw schedules keep typed rejects. |
| 3 | Recipe/JoinSig contract | The existing Recipe and JoinSig expose ordered nested segments and exit transfers without reconstructing meaning from AST/MIR or names. |
| 4 | Physical consumer contract | The existing physical adapter can consume every co-sealed segment/exit and finish or discard the whole session; no partial static argument effects are allowed. |
| 5 | Negative ownership matrix | Foreign root/brand, parent drift, missing/duplicate exit, wrong target, omitted child, extra loop, wrong condition, and route overlap reject before catalog/effects. |
| 6 | Parent handoff | Only after tasks 1–5 are accepted may the parent static card observe one Cataloged row and later design its Selected/retirement evidence. |

The implementation child is
`MIR-CALL-PARSER-LOOPCOND-SOURCE-HANDOFF-I0`. It may edit only the existing
LoopCond source/Recipe/physical owners and their focused guards. Its closeout
must remove the selected static tuple's retained compatibility disposition in
the same bounded series after positive and negative evidence; it may not widen
the parser corpus or reopen VM/legacy routes.

## Stop boundary and acceptance

This D0 is complete as a design stop. The implementation child must still
fail before Cataloged if the co-sealed forest/exit/source product cannot be
consumed by the existing LoopCond physical pipeline. If that occurs, retain
`GenericLoopV1NotSelected`, preserve the compatibility edge, and record the
missing field/consumer as a typed `NoSafeSlice`; do not add a third owner.

The positive acceptance shape is deliberately one source invocation: the
merged parser route reaches the existing LoopCondBreakContinue source product
with all three loop members and all exits co-sealed, then the parent static tuple may
continue to Cataloged. It does not claim physical lowering, source-to-exe,
compatibility retirement, or any other parser callsite.

## Evidence used

- `mir-call-static-compatibility-catalog-target-i0-2026-09-19.md` — named
  `GenericLoopV1NotSelected` terminal and selected static tuple.
- `mir-call-parser-loop-forest-exit-coseal-d0-2026-09-19.md` — resolver forest
  and exit evidence plus the missing callable co-seal.
- `mirbuilder-callable-loop-ready-generic-loop-v1-recipe-authority-d0-2026-08-22.md`
  — existing GenericLoop boundary that rejects this state-machine profile.
- `src/mir/builder/control_flow/facts/loop_cond_break_continue.rs` and
  `src/mir/builder/control_flow/plan/loop_cond/break_continue_item.rs` — the
  existing Facts/Recipe shape that carries ProgramBlock and NestedLoopDepth1.
- `lang/src/compiler/parser/program/parser_program_box.hako:81-189` — finite
  nested/exit-driven counterexample shape.
