# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D7

Status: design_stop__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-ME-RECEIVER-SITE-S0
  (landed — located `Me`/`This` MethodCall receivers consume the
  registered `Receiver` site through `exact_source_receiver_value`;
  json advanced past `incomplete-consumption`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D6 selection lineage.

## Problem

ME-RECEIVER-SITE-S0 landed: `JsonStreamAggregator.ingest`'s loop body
`me.ingestLine(...)` now consumes its `Receiver` site and `ingest`'s
`finish` passes. The app advances to the next named terminal:

```text
lexical scope body failed:
[freeze:contract][static-call/legacy-fallback-retired]
owner=JsonLine method=stringField arity=2
```

The failing call is `JsonLine.stringField(line, "user")`
(`apps/json-stream-aggregator/main.hako:122`), a same-module static-box
method call in a `local` initializer inside `ingestLine`. The retired
legacy fallback is the honest stop; select the owner for issuing this
call.

## Fresh class map (receipt after ME-RECEIVER-SITE-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/route-not-front-selected SourceCallOutsideSelectedFamily` | 3 | boxtorrent_mini (`seedBlocks`), binary_trees (`iterationCheck`), mimalloc_lite (`seedBlocks`) — D5-sealed forks |
| `static-call/legacy-fallback-retired` | 1 | json_stream_aggregator (`JsonStreamAggregator.ingestLine` → `JsonLine.stringField/2`, :122) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `opt` type error (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

## Questions

1. Which contract owns `JsonLine.stringField(line, "user")` in
   `ingestLine`'s body — the selected same-module static-call route
   (`target_for_source` / `StaticResultPublication`), the qualified
   preflight `first_family` admission, or another existing issuer?
2. Why is `JsonLine.find` inside `ingest`'s loop admissible (singleton
   anchor) while `JsonLine.stringField` in `ingestLine`'s ordinary body
   reaches `legacy-fallback-retired`? Is the difference loop-item vs
   statement position, or `ingestLine`'s own selection/admission state?
3. Does `me.ingestLine(...)` in the loop still count as an uncovered
   call inside the singleton-anchor contract (i.e. would the same body
   stop at `SourceCallOutsideSelectedFamily`/`UnconsumedSelected` later
   even if `stringField` issues)?
4. Bounded candidates (order only — each needs its own census):
   - H1: selected static-call issuance for ordinary body statements
     (`JsonLine.stringField`/`intField`/`boolField` in `ingestLine`).
   - F3b: `UnconsumedSelected` publication consumption on the
     singleton LoopCond route.
   - F3c: `SourceItemsMissing` call-free admission.
   - coverage forks: B3-ArrayPush and ordinary-instance-call coverage
     (both D5-sealed; reopen only through their own cards).

## Boundary

- Includes: owner census for `static-call/legacy-fallback-retired` on a
  same-module static-box method call in a selected instance-method
  body; the relationship between ordinary-body static calls and the
  loop singleton-anchor contract; one Decision with a six-line brief or
  sealed NoSafeSlice.
- Excludes: implementation; reopening the D5-sealed coverage forks;
  backend toolchain; inference panic family; NamedArray family.

## Exit

- [ ] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.
