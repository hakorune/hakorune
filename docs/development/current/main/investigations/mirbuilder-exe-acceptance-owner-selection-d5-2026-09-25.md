# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D5

Status: open__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-NO-EXIT-S0
  (landed — disjoint `loop_cond_no_exit` extractor feeding
  `LoopCondBreakContinueFacts` with `NoExitBody`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D4 selection lineage.

## Problem

LOOP-COND-NO-EXIT-S0 landed: all three `callable-loop/facts-absent`
entries cleared and now stop at
`[freeze:contract][callable-loop/route-not-front-selected]
LoopCondRouteRejected(SourceCallOutsideSelectedFamily)`. Select the
next bounded design slice from the remaining 7 failing real-app EXE
entries.

## Fresh class map (receipt after LOOP-COND-NO-EXIT-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/route-not-front-selected SourceCallOutsideSelectedFamily` | 3 | boxtorrent_mini (`HakoAllocPage.seedBlocks/0`, imported), binary_trees (`BinaryTreesBench.iterationCheck/3`), mimalloc_lite (`seedBlocks`) |
| `callable-loop/parts loop-cond-item-unsupported` | 1 | json_stream_aggregator (`ConditionalUpdateIf` in `JsonStreamAggregator.ingest`, main.hako:145) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `no_lowering_variant` (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

`SourceCallOutsideSelectedFamily` fires from
`normal_callable_loop_source_route_items.rs::into_selected_relation`
— every resolver method-call item under the loop must be covered by
either one exact same-module static "selected" publication relation
(singleton contract) or resolver-issued CoreMethod rows covering ALL
items. Loop-internal calls in the failing bodies are instance or
builtin-box calls (`free_stack.push`, `builder.make`,
`positive.itemCheck()`, `me.bump`) that carry neither row.

## Observed coverage sub-boundaries (main-thread probes)

- call-free body `loop(i<n){i=i+1}` in a callable →
  `SourceItemsMissing`
  (`normal_callable_loop_source_route.rs:257` requires ≥1 item).
- instance/builtin-only body (`me.bump(i)`) →
  `SourceCallOutsideSelectedFamily`.
- singleton-covered body (`acc = acc + Step.twice(i)`) → route
  issues, lowering runs, then atomic commit fails at
  `StaticResultPublicationResidual(UnconsumedSelected)` — the
  publication handoff for loop-internal static calls is only
  pre-taken when `has_loop_break_composite_source_candidate(site)`
  (`raw_loop_child_port.rs:88-95`), so non-composite loops never
  install the row into the callable ledger for the body emitter.

## Questions

1. Do the three `SourceCallOutsideSelectedFamily` entries share one
   coverage-authority gap (instance/builtin method calls inside loop
   bodies hold no issued coverage row), or do they fork (ArrayBox
   builtin vs user-box instance receiver)?
2. Is the singleton-plus-anchor contract (`into_selected_relation`
   accepts one `selected` and does not verify remaining items) the
   intended coverage semantic, or is per-item coverage the contract
   the failing loops should satisfy? json reached the parts stage only
   because `JsonLine.find` supplied the anchor — `me.ingestLine` and
   `stream.substring` were never verified.
3. Which class has a bounded slice: single owner + fail-fast tuple +
   acceptance coverage? Candidates:
   - F3a: extend loop-source item coverage so instance/CoreMethod
     rows issued for the body satisfy `into_selected_relation`
     (route-contract layer).
   - F3b: publication consumption for loop-internal static calls
     when no composite loop-break candidate exists
     (`consume_publication` gating).
   - F3c: `SourceItemsMissing` for call-free no-exit bodies
     (vacuous-coverage semantics).
   - F2 (already inventoried): `ConditionalUpdateIf` arm in the
     located-source parts driver — unblocks json but does not move
     the three `SourceCallOutsideSelectedFamily` entries.

## Boundary

- Includes: owner census for the 3-entry
  `SourceCallOutsideSelectedFamily` class; divergence check vs
  `SourceItemsMissing` and `UnconsumedSelected`; whether the
  singleton anchor is the intended contract; one Decision with a
  six-line brief or NoSafeSlice naming the highest-information class.
- Excludes: implementation; backend toolchain; inference panic
  family; NamedArray source-demand family; F2 implementation
  (inventoried, stays next-in-line after the coverage decision).

## Exit

- [ ] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.
