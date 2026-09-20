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
