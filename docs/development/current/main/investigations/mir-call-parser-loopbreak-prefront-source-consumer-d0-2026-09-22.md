---
Status: design_stop__2026-09-22__PrefrontLoopBreakSourceConsumer
Task: MIR-CALL-PARSER-LOOPBREAK-PREFRONT-SOURCE-CONSUMER-D0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
Implementation permission: false; design only until the source owner and terminal boundary are accepted
NextCard: MIR-CALL-PARSER-LOOPBREAK-COMPOSITE-SOURCE-CUTOVER-I3
---

# Parser pre-front LoopBreak source consumer D0

## Six-line brief

```text
Decision: keep I3 publication acceptance queued until the finite preceding
  loop rows have a source-backed owner or an explicit dependency terminal.
Source authority + canonical issuer: resolver loop forest/exit ledger and
  issue_callable_loop_break_source_facts_v1; the package issuer owns complete
  row classification, but no current owner lowers the pre-front rows.
Non-authority: SupportedNonCandidate as route admission, route-selector output
  as a lowering receipt, names/line numbers/AST rescans, LoopRouteContext,
  legacy composer re-entry, generic retry/fallback, VM, or a new authority.
Fail-fast boundary: before Builder effects, every in-scope row must have an
  exact source owner and route relation; unsupported rows remain named stops.
Smallest next slice: decide one existing-owner extension for the finite
  pre-front inventory, or explicitly retain those rows as dependency terminals.
Non-claims: no code, fixture, publication, caller switch, old-edge deletion,
  backend parity, warning cleanup, or I3 completion.
```

## Read-only decision — 2026-09-22

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
| `ParserStringUtilsBox.trim/1:87` | LoopBreak/non-candidate | no source route consumer; named dependency terminal |
| `ParserStringUtilsBox.to_int/1:105` | LoopBreak/non-candidate | no source route consumer; named dependency terminal |
| `ParserDelegateExposesBox._parse_delegate/3:54` | LoopBreak/non-candidate | no source route consumer; named dependency terminal |
| `ParserRecordDeclarationBox.parse/3:19` | LoopBreak/non-candidate | no source route consumer; named dependency terminal |

The line numbers are source witnesses from the bounded parser import closure;
they are not selector inputs. The source/owner relation comes from the
resolver-issued forest and package batch slot.

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

## Bounded decision required before implementation

The next design pass must choose one of these two explicit outcomes for the
four non-candidate rows:

1. extend one existing source route owner with an exact resolver/source-port
   product and focused positive/negative coverage; or
2. retain them as named dependency terminals and narrow the selected parser
   acceptance boundary accordingly.

Until that choice is accepted, do not alter route selection, add a generic
fallback, re-enter `LoopRouteContext`, reorder package declarations, or issue
an empty/default semantic receipt. I3 task 4 publication acceptance remains
queued behind this D0; task 5 caller switch and R0 task 6 caller-zero/deletion
remain unopened.

