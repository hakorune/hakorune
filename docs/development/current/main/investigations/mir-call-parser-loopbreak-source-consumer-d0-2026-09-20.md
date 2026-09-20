---
Status: design_stop__2026-09-20__ParserLoopBreakSourceConsumer
Task: MIR-CALL-PARSER-LOOPBREAK-SOURCE-CONSUMER-D0
Current execution row: MIR-CALL-PARSER-LOOPBREAK-SOURCE-CONSUMER-D0
Date: 2026-09-20
Parent: mir-call-parser-source-to-mir-package-acceptance-window-d0-2026-09-20.md
Implementation permission: false; this row fixes the missing owner contract before code or fixture changes
---

# Parser LoopBreak source consumer D0

## Six-line brief

```text
Decision: keep the parser package acceptance stop closed until the same
  invocation's LoopBreak dependency has a source-lineage consumer that keeps
  whole-package coverage intact.
Source authority + canonical issuer: the resolver forest/exit ledger and the
  existing LoopBreakFacts/Recipe outcome for the finite dependency methods;
  no source-aware LoopBreak issuer currently exists, so this row must name
  the extension owner before implementation.
Non-authority: selected-only lowering, method-name or ordinal filtering,
  package cloning, AST/MIR rescans, LoopRouteContext reconstruction, GenericLoop
  retry, compatibility fallback, or a synthetic Cataloged/Selected receipt.
Fail-fast boundary: reject before Builder effects when the source caller,
  exact exit/forest relation, LoopBreak shape, package brand, or physical
  obligation is missing, foreign, duplicated, or mismatched.
Smallest next slice: inventory the LoopBreakRecipe front in the same merged
  parser package, then choose one existing LoopBreak Facts/Recipe/physical
  owner for a move-only source input and an exclusive old-edge delete-set.
Non-claims: no parser source-to-MIR success, publication, production switch,
  old-edge deletion, fallback, VM/AOT parity, or warning cleanup.
```

## Exact frontier

The selected package remains the bounded parser invocation:

```text
caller       = ParserProgramBox.parse/2
target       = ParserStringUtilsBox.starts_with/3
target site  = resolver-issued site for parser_program_box.hako:102
route        = LoopCondBreakContinue with the selected LoopTrue child
result       = ExactI64, required ordinal `[1]`
package stop = [freeze:contract][callable-loop/route-not-front-selected]
               GenericLoopV1NotSelected
               raw front = [LoopBreakRecipe]
```

The stop is dependency evidence. It is not permission to lower only the
selected `starts_with/3` method. The existing root work plan iterates every
immediate and deferred method, and the installed package's selected-call APIs
are scoped loans whose `complete()` requires all selected coverage.

## Finite dependency census

The parent nested-loop design already fixes the only reviewed source topology
that can produce this package boundary. The source witness is one caller,
`ParserProgramBox.parse/2`, with these loop members:

| Member | Source witness | Exit witness |
| --- | --- | --- |
| root state-machine loop | `:81 loop(cont_prog == 1)` | `break` at `:85/:95`; returns at `:104/:109/:127/:142/:161/:165/:194` |
| static-semicolon child | `:131 loop(static_semis == 1)` | condition-only child boundary |
| semicolon-scan child | `:182 loop(true)` | `continue :186` and `break :188` |

The parent card records the resolver-owned root/child indices, frame keys, and
all exit records; its line numbers are source witnesses only. Two terminal
observations are currently visible and must not be conflated: the lifecycle
raw front reports `[LoopBreakRecipe]`, while the source bridge normalizes its
stop to `GenericLoopV1NotSelected`. Neither observation is a source-to-route
slot map, and no co-sealed relation currently proves that they identify the
same dependency rows. Therefore this D0 must not infer either label from
names, line numbers, or AST shape. The next audit has a finite target: recover
the exact resolver rows for those three members from the same invocation, then
decide whether the existing generic-direct LoopBreak product can receive them.

The existing LoopBreak design explicitly rejects the parser nested profile as a
new route: its logical product is generic-direct-only, while its physicalizer
is `NoSafeSlice` because the legacy composer enters `lower_loop_v0` after
Builder-bound mutation. This row may reopen that owner only with a source-aware
move-only input and a builder-free pre-effect consumer contract; it may not
promote a parser-specific LoopBreak route.

## Resolver/consumer bridge audit

The resolver-side API is intentionally source-complete but route-neutral:
`VerifiedResolvedFunctionV1::loop_sites()` exposes the finite Loop-site
inventory, `resolved_loop_source_forest(root)` co-seals parent indices, and
`resolved_loop_source_context(site)` lends the exact Scope/Region pair. None of
these products records which physical `LoopRouteId` later classified the site.
`only_loop_site()` is a singleton guard and cannot select one member from this
package's three-member forest.

The current source bridge has the complementary boundary: its route issuer
accepts only Generic/LoopCond/LoopTrue source products. The LoopBreak path
starts at `LoopBreakFacts`, enters `route_loop_break_recipe` with
`LoopRouteContext`, and its composer reaches `lower_loop_v0` after it has
already received a mutable `MirBuilder`. There is no source-port argument or
package route inventory at that boundary. The package root therefore cannot
name the exact method behind the raw `[LoopBreakRecipe]` observation without
adding a new route observer or inferring from names/AST, both prohibited here.

This is a confirmed missing-owner condition, not a missing line-number lookup.
The next slice must first name one existing package-scoped route inventory
authority (or explicitly reject that owner) before any LoopBreak source issuer
or physical adapter is designed. Until then, `GenericLoopV1NotSelected` stays
the terminal and the selected parser tuple cannot advance to Cataloged.

## Owner audit and NoSafeSlice decision

The existing package exposes source AST/catalog loans, selected callable loans,
physical-signature rows, and result contracts. None is a package-scoped
acceptance observer that can consume the selected tuple while retaining the
LoopBreak dependency as a named terminal. The LoopBreak path still uses
`LoopBreakFacts`, mutation-first `loop_break_composer`, and
`route_loop_break_recipe` with `LoopRouteContext`/`MirBuilder`; it has no
source-lineage co-seal or source-port consumer.

Therefore this row stays `design_stop` with `NoSafeSlice`. The package
acceptance-window card is handed off without a parser acceptance claim. The
next design work is bounded to the finite LoopBreak dependency inventory below;
it must not reopen the parked route generally.

## Required next-design inventory

| Item | Required evidence |
| --- | --- |
| Dependency callers | exact methods represented by the same merged package's `LoopBreakRecipe` front |
| Source issuer | resolver forest/exit ledger relation co-sealed with the existing LoopBreak facts, or an explicit missing-issuer decision |
| Physical consumer | one existing LoopBreak owner receiving one move-only source input and reusing cleanup/verifier |
| Failure boundary | named pre-effect reject for missing, foreign, duplicate, out-of-root, or mismatched source rows |
| Retirement | finite old caller/edge inventory and an exclusive delete-set; no caller-zero claim before cutover |

If the finite inventory cannot satisfy these conditions in an existing owner,
record the missing owner and keep `GenericLoopV1NotSelected` as the terminal.
Do not add a second Recipe, route, ledger, or compatibility fallback to make
the parser tuple appear green.

## Acceptance boundary

This design row can advance only after the inventory names one source issuer,
one physical consumer, one pre-effect terminal, and one exclusive deletion set.
Focused positive/negative evidence must cover exact source site, package brand,
exit/forest relation, duplicate/foreign rows, and dependency terminality. Until
then the parent parser tuple remains a design dependency and all production,
publication, and legacy-retirement claims stay closed.
