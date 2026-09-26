# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D6

Status: decision-pending__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-COND-UPDATE-S0
  (landed — `ConditionalUpdateIf` located-source parts arm through the
  existing `try_lower_conditional_update_if_input` facade; json advanced
  past `loop-cond-item-unsupported`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D5 selection lineage.

## Problem

LOOP-COND-COND-UPDATE-S0 landed: json_stream_aggregator now lowers the
`ConditionalUpdateIf` item in `JsonStreamAggregator.ingest` and stops at
a new named terminal:

```text
[freeze:contract][callable-semantic-lowering/incomplete-consumption]
owner=FunctionOwnerIdV1 { compilation: 1, slot: 27 } entry=true
locals=3/3 variables=11/12
missing_variables=[SourceNodeSiteV1([Body(2), LoopBody(2), Receiver])]
assignments=2/2 lambdas=0/0 loop_break_transport_kind=None
```

`LoopBody(2)` is `me.ingestLine(stream.substring(start, end))` — the
`me` receiver site inside the loop body was never consumed by any
lowering product. Select the next bounded design slice.

## Fresh class map (receipt after LOOP-COND-COND-UPDATE-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/route-not-front-selected SourceCallOutsideSelectedFamily` | 3 | boxtorrent_mini (`seedBlocks`), binary_trees (`iterationCheck`), mimalloc_lite (`seedBlocks`) — D5-sealed forks |
| `callable-semantic-lowering/incomplete-consumption` (Receiver site) | 1 | json_stream_aggregator (`JsonStreamAggregator.ingest`, LoopBody(2) `me.ingestLine`) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `opt` type error (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

## Questions

1. Which contract owns the `Receiver` site of a `me.method(...)` call
   inside a loop body — the same DeclaredInstance receiver-consumption
   lane (parked `MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0`),
   or a distinct consumption-ledger gap in
   `callable-semantic-lowering`?
2. How did the singleton-anchor route carry `me.ingestLine`? Per D5 the
   singleton contract admits one selected static call (`JsonLine.find`)
   without verifying remaining items — was the expression statement
   issued as an item (and which kind) or never issued, leaving its
   `Receiver` variable site unconsumed?
3. Does the consumption audit expect every `Receiver` site to be
   consumed by a lowering product, or is the audit itself missing a
   receiver-consumption record that the located dispatcher should emit
   when it lowers a `me.method` item?
4. Bounded candidates (order only — each needs its own census):
   - G1: receiver-site consumption for loop-body `me.method` items
     (the json blocker).
   - F3b: `UnconsumedSelected` publication consumption on the
     singleton LoopCond route.
   - F3c: `SourceItemsMissing` call-free admission.
   - coverage forks: B3-ArrayPush and ordinary-instance-call coverage
     (both D5-sealed; reopen only through their own cards).

## Boundary

- Includes: owner census for the `Receiver`-site consumption terminal;
  the item-issuance question for uncovered call statements under a
  singleton anchor; relation to the parked DeclaredInstance lane; one
  Decision with a six-line brief or sealed NoSafeSlice.
- Excludes: implementation; reopening the D5-sealed coverage forks;
  backend toolchain; inference panic family; NamedArray family.

## Exit

- [ ] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.
