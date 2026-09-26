# MIRBUILDER-EXE-ACCEPTANCE-SOURCE-CALL-COVERAGE-D14

Status: design_stop__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  OWNER-SELECTION-D13 / DEAD-ROUTE-SOLE-FAMILY-S1 landed
Mode: design_stop — census and Decision only.

## Blocking observation

```text
EXE (pure-first, debug binary):
  [freeze:contract][callable-loop/route-not-front-selected]
  LoopCondRouteRejected(SourceCallOutsideSelectedFamily {
    call_sites: [
      SourceExprSiteV1([Body(2), LoopCondition, Rhs, Lhs, Lhs]),
      SourceExprSiteV1([Body(2), LoopCondition, Rhs, Rhs, Lhs])
    ] })
  function=StringHelpers.trim/1
  source=Cataloged(StaticBoxMethod "StringHelpers"."trim"/1)
  site=[Body(2)]

VM (debug binary): unchanged known terminal —
  loop winner selection declined: zero selected family
  candidates fn=JsonStreamAggregator.ingest/1 [Body(2)]
  (ledger-less instance method → route_loop spine;
  separate authority, recorded non-claim).
```

`trim`'s whitespace loop now front-selects
`LoopCondBreakContinue` (S1). The route token's
source-call coverage gate rejects: the two `s.substring`
calls live inside `LoopCondition` (`Rhs.Lhs.Lhs`,
`Rhs.Rhs.Lhs` — the object positions of the two
`substring(...) == literal` terms) and have no selected
static publication relation.

## Census questions

- Which authority owns call sites inside `LoopCondition`?
  `CallableLoopSourceTargetProbeV1::from_parts` /
  `into_item_dispositions` covers resolver *items* (body
  statements); do condition-position call sites have an
  existing coverage arm (selected static publication rows,
  resolver `CoreMethod` rows) or is condition-call coverage
  an unclaimed gap in the D5 family?
- Is `s.substring` resolvable as an exact static target or
  a `CoreMethod` row at this site (String haystack —
  string method on a parameter binding)?
- Does the existing probe intentionally exclude
  `LoopCondition` sites (bounded scope) or is the site
  filter missing this position? Compare with
  `binary_trees.iterationCheck` /
  mimalloc `seedBlocks` — same
  `SourceCallOutsideSelectedFamily` family (D5 inventory)
  at body positions.
- What does `SourceItemsMissing` vs
  `SourceCallOutsideSelectedFamily` each own — is
  condition coverage already the designed owner and the
  site registration incomplete, or vice versa?

## Boundary of this census

- Covers: `SourceCallOutsideSelectedFamily` on
  condition-position call sites for the LoopCondBreakContinue
  owner (`trim`-class), from `CallableLoopSourceTargetProbeV1`
  input assembly to the reject emission.
- Excludes: `ConditionalUpdateIf` parts boundary
  (`ingest`, F2); VM ledger-less lanes; winner spine;
  continue-only loops; body-position call coverage already
  green.

## Decision

(to be filled after census)

## Exit

- [ ] Failing call sites' coverage authority identified.
- [ ] Existing owner vs unclaimed gap classified.
- [ ] One bounded S-card emitted or NoSafeSlice recorded.
