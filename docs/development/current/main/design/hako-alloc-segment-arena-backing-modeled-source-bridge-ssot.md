# Hako Alloc Segment Arena Backing Modeled Source Bridge SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Record a modeled backing source bridge from an accepted MIMAP-256A modeled
arena-slot report without creating real raw pointer residence, pointer-derived
lookup, or real arena backing.

The source bridge is a scalar/model inventory fact. It is not a runtime memory
provider, not an OSVM/page-source call, not a pointer lookup key, and not an
arena backing allocation.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_bridge_box.hako
```

The owner may:

- observe accepted modeled arena-slot reports;
- preserve segment id, arena id, arena-slot token, and slot geometry;
- preserve modeled source kind, token, capacity, committed bytes, and
  alignment;
- reject missing/rejected inputs, invalid arena-slot token, invalid source
  shape, invalid geometry, and closed-substrate requirements.

The owner must not:

- create real raw pointer residence;
- perform pointer-derived lookup or dereference;
- allocate real arena backing;
- mutate a real segment-map;
- execute atomic bitmap claims;
- call page-source or OSVM seams;
- infer anything from owner names or backend matchers.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | modeled source bridge accepted |
| `1` | modeled arena-slot report missing |
| `2` | modeled arena-slot report was rejected |
| `3` | arena-slot token invalid |
| `4` | source shape invalid |
| `5` | arena-slot geometry invalid |
| `6` | closed-substrate requirement present |

## Validation

MIMAP-260A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh --level L2
```

The guard must:

- prove accepted modeled source bridge publication;
- prove missing/rejected arena-slot rejection;
- prove invalid arena-slot token, source shape, geometry, and
  closed-substrate rejection;
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
