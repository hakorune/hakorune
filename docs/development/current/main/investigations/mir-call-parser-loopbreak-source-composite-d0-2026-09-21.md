---
Status: ParkedSealed__NoSafeSlice__2026-09-21__ParserLoopBreakCompositeSourceOwner
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-COMPOSITE-D0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-SOURCE-COMPOSITE-D0
Date: 2026-09-21
Parent: mir-call-parser-loopbreak-source-physical-i0-2026-09-21.md
Implementation permission: false; design and finite source-shape census only
NextCard: MIR-CALL-D1B-DIRECT-CALL-SOURCE-INVENTORY-COSEAL-D0
---

# Parser LoopBreak composite source owner D0

## Six-line brief

```text
Decision: keep the direct three-statement LoopBreak owner closed and decide
  the parser's composite LoopBreak shape inside the existing LoopBreak family;
  do not reclassify it to LoopCond merely to reach a consumer.
Source authority + canonical issuer: the resolver loop forest/exit ledger and
  the existing LoopBreakFacts/Recipe issuer (`issue_loop_break_source_projection_v1`
  -> `issue_callable_loop_break_source_facts_v1`).
Non-authority: LoopRouteContext, AST/name re-scan, route-name inference, a second
  Facts/Recipe/JoinSig issuer, generic retry, compatibility fallback, VM/backend,
  or a fixture that bypasses the package boundary.
Fail-fast boundary: exact callable/root/body/exit relation, complete source item
  coverage, nested branch ownership, carrier/step relation, and selected target
  relation must be proven before any physical frame or effect.
Smallest next slice: map the finite parser body to the existing LoopBreak
  Recipe vocabulary and source port; choose a same-owner source Recipe extension
  or seal `NoSafeSlice` with the missing relation/issuer named.
Non-claims: no code, fixture, source-to-MIR acceptance, publication, caller
  switch, old-edge deletion, backend parity, warning cleanup, or VM work.
```

### Census boundary

`ParserProgramBox.parse/2` loop sites at `parser_program_box.hako:81` and `:182`
are the finite selected boundary. It includes the body statements, nested
conditional/exit branches, resolver loop-forest members, carrier updates, and
the `starts_with/3` target sites that occur before package completion. It excludes
imported parser callables, unrelated compatibility loops, the already-closed
direct three-statement LoopBreak row, and the LoopTrue physical row at `:182`
when its literal-true owner remains selected.

## Current evidence

The direct source adapter and `build_loop_break_source_recipe` accept only the
exact three-statement layout with a direct break-if and final step. The parser
site at `:81` instead has a whitespace assignment, nested conditional exits,
progress/guard assignments, and nested method-call observations. The source
LoopBreak issuer has no source topology or recipe item product for those nested
rows and explicitly treats the corresponding projection shapes as unsupported.

The route registry also gives LoopBreak precedence: `pred_loop_cond_break_continue`
is disabled when LoopBreak Facts are present. The current `GenericLoopV1NotSelected`
terminal is therefore a truthful missing physical owner, not permission to route
the same body through the LoopCond consumer. The sibling loop at `:182` is a
LoopTrue shape and remains outside this row's owner decision.

## Design alternatives

### Same-owner source Recipe extension

The existing LoopBreak Facts/Recipe owner may be extended only if it can retain
the resolver-located statement and child-body relation for every composite item,
including nested if branches, all break exits, carrier/step assignments, and
the method target relation. The physical consumer must then reuse
`CallableLoopSourcePartsBlockV1` and the existing `lower_loop_v0_core` spine,
with all validation before frame allocation. This option may add no second
semantic issuer and may not synthesize `StmtRef` or AST nodes from names.

### NoSafeSlice

If the existing LoopBreak Facts/Recipe cannot express the composite body without
losing source identity, or if the resolver forest does not issue the required
nested exit/item relations, keep this row `NoSafeSlice`. The package terminal and
`GenericLoopV1NotSelected` boundary remain named; no empty candidate, fallback,
LoopCond reclassification, or compatibility retry may be introduced.

## Acceptance for this D0

This design row closes only when the finite two-site census names the existing
Facts/Recipe fields and resolver source relations for every required item, then
records either the same-owner extension contract or `NoSafeSlice` with owner,
evidence, and an observable reopen trigger. No Rust, Hako, fixture, guard,
production caller, or semantic receipt may change in this D0.

## Decision — NoSafeSlice

The finite census is complete for the two selected parser sites. The existing
`LoopBreakFacts` product carries loop expressions and an optional direct
three-site `source_topology`; the specialized LoopBreak extractors leave that
topology absent. `build_loop_break_source_recipe` likewise requires the exact
three-statement body, while the source issuer filters composite bodies before a
source candidate is issued. Finally, the route registry suppresses the
LoopCond source route whenever LoopBreak facts are present. These facts make the
`GenericLoopV1NotSelected` terminal at `parser_program_box.hako:81` a truthful
missing physical owner rather than permission to reclassify the body.

Therefore this row has no safe implementation slice using the current semantic
products. The missing relation is a source-bound composite LoopBreak body/item
and nested-exit topology co-sealed by the existing LoopBreak Facts/Recipe owner
before physical allocation. Adding it would be a new source product and needs a
separate accepted design decision; routing through LoopCond, synthesizing names
or AST nodes, or issuing an empty candidate is forbidden. The row is closed as
`ParkedSealed__NoSafeSlice` with owner `CallableGenericLoopSourceFactsIssuerV1`
and its LoopBreak Facts/Recipe physical consumer.

Reopen only when a bounded same-owner composite product can preserve every
resolver-located body item, nested exit, carrier/step relation, and target site
through one Recipe and one physical input, or when an explicit typed pre-effect
retirement decision for the parser dependency is accepted. A local green test,
route-name change, compatibility retry, or backend result cannot reopen it.

## Reopen trigger and non-authority

Reopen the implementation row only after one source-bound composite Recipe and
physical input can be co-sealed in the existing owner, or after an explicit
typed pre-effect retirement decision for the parser dependency. A local green
test, a route name, a LoopRouteContext projection, an AST rescan, a default
receipt, or a backend-specific result is not an issuer and cannot reopen this
row.
