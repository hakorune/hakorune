# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D4

Status: open__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-ORDINARY-NEW-BIRTH-SITE-INDEX-S0
  (landed — destination-less birth-recipe site index)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0/D1/D2/D3 selection lineage (D2 was NoSafeSlice).

## Problem

The birth-recipe site index landed: all three
`ordinary-new/birth-global-legacy-stopped` entries cleared and now
stop at `[freeze:contract][callable-loop/facts-absent]`. Select the
next bounded design slice from the remaining 7 failing real-app EXE
entries.

## Fresh class map (receipt after BIRTH-SITE-INDEX-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/facts-absent` | 3 | boxtorrent_mini, binary_trees, mimalloc_lite |
| `callable-loop/parts loop-cond-item-unsupported` | 1 | json_stream_aggregator (`ConditionalUpdateIf` cond in a `JsonLine` static child) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `no_lowering_variant` (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

`facts-absent` fires from `raw_loop_child_entry.rs:541` — the
callable lane reaches a `loop(...)` statement whose loop Facts were
never issued for that callable. All four non-singleton entries are
loop-family but present different evidence shapes: `facts-absent`
(no facts issued at all for the callable) vs
`loop-cond-item-unsupported` (facts issued, `ConditionalUpdateIf`
item shape rejected).

## Questions

1. Do the three `facts-absent` entries share one issuer gap (loop
   Facts not issued for constructor/child callables), or do they
   fork into distinct loop-source families?
2. Is `loop-cond-item-unsupported` the next terminal past
   `facts-absent` once facts exist, i.e., would fixing the issuer
   merely move all four entries to the same item-shape boundary?
   If so, which class owns the bounded first slice?
3. Which class has a bounded slice: single owner + fail-fast tuple
   + acceptance coverage?

## Boundary

- Includes: owner census for the 3-entry `facts-absent` class;
  divergence check vs `loop-cond-item-unsupported`; one Decision
  with a six-line brief or NoSafeSlice with the highest-information
  class named.
- Excludes: implementation; backend toolchain (`no_lowering_variant`
  stays parked-debt); inference panic family (prior D0 lineage);
  NamedArray source-demand family.

## Exit

- [ ] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.
