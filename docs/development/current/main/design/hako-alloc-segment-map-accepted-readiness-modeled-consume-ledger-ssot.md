# Hako Alloc Segment Map Accepted Readiness Modeled Consume Ledger SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-157A composes the accepted explicit-ID segment-map readiness path into
the existing modeled consume / ledger lane.

The row proves:

```text
MIMAP-153A accepted readiness
  -> MIMAP-091A modeled consume
  -> MIMAP-094A modeled ledger row
```

This is still model-space. It does not allocate real segment memory or mutate a
real segment map.

## Owner

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
```

The owner is intentionally a thin composition owner. It delegates:

- readiness production to MIMAP-153A proof output;
- modeled consume arithmetic/token shape to MIMAP-091A;
- ledger append/live-count/token lookup shape to MIMAP-094A.

## Validation

MIMAP-157A uses L2 daily validation:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_accepted_readiness_modeled_consume_ledger_guard.sh
```

The guard runs VM proof, MIR JSON assertions, and pure-first route preflight.
It does not build or run the EXE. L3 EXE evidence is deferred to a future
consume-ledger closeout pack unless a new backend route shape is introduced.

## Stop Lines

- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real segment allocation/free.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Next

The next selected row is:

```text
MIMAP-158A segment-map modeled consume ledger diagnostics
```

MIMAP-158A should add blocked / duplicate / stale observer coverage around the
same modeled consume ledger boundary without opening raw substrate.
