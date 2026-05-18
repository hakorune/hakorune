# Hako Alloc Segment Arena Backing Modeled No-Escape Address Residence Diagnostics SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Publish observer-only diagnostics for the MIMAP-248A modeled no-escape address
residence inventory before closing out the residence family.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_no_escape_address_residence_diagnostic_box.hako
```

The owner may:

- observe MIMAP-248A inventory counters;
- mirror the last residence report reason;
- publish seen flags for missing / rejected / invalid / escape / closed
  substrate reject categories.

The owner must not:

- record residence rows;
- create real raw pointer residence;
- perform pointer-derived lookup or dereference;
- allocate real arena backing;
- mutate a real segment-map;
- execute atomic bitmap claims;
- call page-source or OSVM seams;
- infer anything from owner names or backend matchers.

## Validation

MIMAP-249A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_no_escape_address_residence_diagnostics_guard.sh --level L2
```

The guard must:

- prove diagnostic observation after the MIMAP-248A inventory;
- prove observer counters and seen flags;
- prove inactive execution flags remain zero;
- prove the MIR JSON has typed report fields and the expected route surface.

## Stop Lines

- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.
