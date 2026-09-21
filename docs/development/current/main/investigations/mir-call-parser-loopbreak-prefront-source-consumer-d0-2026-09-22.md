---
Status: closeout__2026-09-22__PrefrontLoopBreakSourceConsumerDecisionAccepted
Task: MIR-CALL-PARSER-LOOPBREAK-PREFRONT-SOURCE-CONSUMER-D0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
Implementation permission: false; accepted design only, implementation is delegated to the successor I0
NextCard: MIR-CALL-PARSER-LOOPBREAK-PREFRONT-STRUCTURED-SOURCE-I0
---

# Parser pre-front LoopBreak source consumer D0

## Six-line brief

```text
Decision: extend the existing LoopBreak source owner to the finite preceding
  structured rows, then return to I3 publication acceptance.
Source authority + canonical issuer: resolver loop forest/exit ledger and
  issue_callable_loop_break_source_facts_v1; the package issuer owns complete
  row classification, but no current owner lowers the pre-front rows.
Non-authority: SupportedNonCandidate as route admission, route-selector output
  as a lowering receipt, names/line numbers/AST rescans, LoopRouteContext,
  legacy composer re-entry, generic retry/fallback, VM, or a new authority.
Fail-fast boundary: before Builder effects, every in-scope row must have an
  exact source owner and route relation; unsupported rows remain named stops.
Smallest next slice: implement one existing-owner structured source product
  with per-item `SelectedStatic`/`CoreMethod` dispositions and residual guards.
Non-claims: no code, fixture, publication, caller switch, old-edge deletion,
  backend parity, warning cleanup, or I3 completion.
```

## Read-only decision before D0 — 2026-09-22

The merged parser package reaches a later composite candidate, but its
lifecycle visits earlier callable rows first. The existing source issuer
accepts only `GenericLoopV1`, `LoopCondBreakContinue`, and
`LoopTrueBreakContinue`. `LoopBreakRecipe`, `LoopSimpleWhile`, and other
non-front routes have no source-aware consumer arm, so the named
`callable-loop/route-not-front-selected` /
`GenericLoopV1NotSelected` terminal is correct.

The package row `SupportedNonCandidate { loop_count }` is a complete-coverage
classification only. It does not issue a route, Recipe, source port, or
permission to re-enter the legacy owner. The compatibility
`route_loop_break_recipe` path requires `LoopRouteContext` and `MirBuilder`
without resolver source identity, so reusing it would create a second
authority and is rejected.

## Finite inventory

| source row | current owner/result | D0 disposition |
| --- | --- | --- |
| `ParserProgramBox.parse/2:81` | composite LoopBreak candidate | selected family; retained for I3 publication |
| `ParserProgramBox.parse/2:131` | nested static-semicolon child | child relation of the root composite Recipe; not an independent caller |
| `ParserProgramBox.parse/2:182` | literal-true LoopTrue | existing LoopTrue source owner already crosses this row |
| `StringHelpers.index_of/3` | LoopCond with bound receiver | existing LoopCond handoff crosses it; static publication is outside this row |
| `ParserStringUtilsBox.trim/1:87` | LoopBreak/non-candidate | finite I0 structured-source inventory |
| `ParserStringUtilsBox.to_int/1:105` | LoopBreak/non-candidate | finite I0 structured-source inventory |
| `ParserDelegateExposesBox._parse_delegate/3:54` | LoopBreak/non-candidate | finite I0 structured-source inventory |
| `ParserRecordDeclarationBox.parse/3:19` | LoopBreak/non-candidate | finite I0 structured-source inventory |

The line numbers are source witnesses from the bounded parser import closure;
they are not selector inputs. The source/owner relation comes from the
resolver-issued forest and package batch slot.

## Source-item relation census

The four pre-front rows cannot be admitted by a `core_methods`-only shortcut.
Their loop bodies contain these resolver-issued call families:

| row | loop-local calls | relation consequence |
| --- | --- | --- |
| `trim/1` | bound `substring` reads | source-port core-method coverage |
| `to_int/1` | bound `indexOf` read | source-port core-method coverage |
| `_parse_delegate/3` | bound `ctx.*` reads plus static `ParserStringUtilsBox.to_int` | core-method coverage plus an exact static-call disposition |
| `ParserRecordDeclarationBox.parse/3` | bound `ctx.*` reads | source-port core-method coverage |

The static `to_int` row must be either joined to an existing selected
publication handoff or rejected as an explicit outside-family obligation. It
must not disappear when the route is classified as `SupportedNonCandidate`.
Likewise, an empty target list is not evidence that a loop has no source
items. The eventual source owner therefore needs one per-item disposition
(`SelectedStatic`, `CoreMethod`, or named rejection) and a residual check
before Parts/LoopV0 lowering.

## Authority audit

```text
resolver forest / exit ledger
  -> issue_callable_loop_break_source_facts_v1
  -> package row:
       Candidate | CompositeCandidate | SupportedNonCandidate |
       Unresolved | Rejected
  -> exact-site candidate take
  -> composite Recipe and existing physical owner
```

The final two steps exist only for the selected candidate. No current product
maps a `SupportedNonCandidate` row to source-aware `LoopSimpleWhile` or
`LoopBreak` lowering. `select_recipe_first_routes` verifies route selection but
does not consume non-selected routes, and `RawLegacyChildLoweringPortV1` is a
compatibility sibling outside the source-backed caller.

## Accepted Decision — 2026-09-22

Use the existing LoopBreak source Facts/Recipe owner and the existing
associated-source Parts/LoopV0 physical owner for the finite pre-front rows.
The source projection/Recipe envelope is extended from multi-member composite
roots to one-member structured roots whose body-role tree contains the explicit
break/branch relation. The direct three-statement candidate remains a separate
contract; no route selector is widened and no `LoopRouteContext` re-entry is
added.

The source item relation is one co-sealed batch issued from the existing
resolver item ledger and target/publication owner. Each item must be exactly
one of:

* `SelectedStatic`: consume the existing one-shot publication handoff and
  retain its exact representation/argument requirement;
* `CoreMethod`: retain the existing resolver-issued bound-receiver item and
  let the existing source expression port lower it without a publication row.

Mixed batches are valid only when every item has one disposition in source
order. `TargetOnly`, missing handoff, foreign item, duplicate item, uncovered
item, and residual publication rows remain named rejects. Empty/default lists
are not accepted. The physical envelope validates this batch before Parts or
LoopV0 allocation and then reuses the existing source port.

This is an extension of one existing owner, not a second semantic authority:
the resolver issues source items, the publication owner issues static result
handoffs, and the existing LoopBreak Recipe/physical owner consumes them.

## Bounded implementation handoff

The successor I0 must:

1. admit the finite one-member structured projections while keeping direct and
   multi-member rows distinct;
2. issue and consume the ordered mixed item-disposition batch with positive,
   missing, foreign, duplicate, target-only, and residual guards;
3. lower through the existing source Parts/LoopV0 owner before effects; and
4. prove the four pre-front parser rows while preserving the current named
   terminal for shapes outside this accepted family.

Only after I0 is green does I3 task 4 publication acceptance reopen.

The earlier two-way question is now resolved in favor of the existing-owner
extension above. Until the successor I0 is green, do not alter route
selection, add a generic fallback, re-enter `LoopRouteContext`, reorder
package declarations, or issue an empty/default semantic receipt. I3 task 4
publication acceptance remains queued; task 5 caller switch and R0 task 6
caller-zero/deletion remain unopened.
