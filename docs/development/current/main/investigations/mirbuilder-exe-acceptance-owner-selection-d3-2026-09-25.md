# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D3

Status: open__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-MAIN-WRAPPER-PARAM-ENTRY-S0
  (landed — injected-local entry adoption)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0/D1/D2 selection lineage (D2 was NoSafeSlice).

## Problem

Wrapper entry adoption landed (`main(args)` installs declared
`Parameter` bindings from the injector's published locals; the lane
now reaches body lowering and publication for arity-bearing mains).
Select the next bounded design slice from the remaining 7 failing
real-app EXE entries.

## Fresh class map (receipt after PARAM-ENTRY-S0)

| class | entries | first named stop |
|---|---|---|
| `ordinary-new/birth-global-legacy-stopped` | 3 | boxtorrent_mini, binary_trees, mimalloc_lite |
| `callable-loop/parts loop-cond-item-unsupported` | 1 | json_stream_aggregator (`ConditionalUpdateIf` cond in a `JsonLine` static child) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `no_lowering_variant` (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

All three `birth-global-legacy-stopped` entries stop in a static
child's `birth` on the ordinary-new lane — lowered before the root
body, so none exercises the just-landed wrapper-param adoption yet.

## Questions

1. Do the three `birth-global-legacy-stopped` entries share one
   owner edge (ordinary-new claim authority for `birth` bodies), or
   do they fork (return-position `new` vs local-initializer `new`
   vs other)?
2. Is `loop-cond-item-unsupported` (ConditionalUpdateIf in cond
   position) the same family as the tracked loop-facts chain, or a
   new class?
3. Which class has a bounded slice: single owner + fail-fast tuple
   + acceptance coverage?

## Boundary

- Includes: owner census for the 3-entry birth class; divergence
  check vs the other three singleton terminals; one Decision with a
  six-line brief or NoSafeSlice with the highest-information class
  named.
- Excludes: implementation; backend toolchain (`no_lowering_variant`
  stays parked-debt); inference panic family (prior D0 lineage).

## Exit

- [ ] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [ ] Next S-card emitted OR the named design card opened;
      pointers synced.
