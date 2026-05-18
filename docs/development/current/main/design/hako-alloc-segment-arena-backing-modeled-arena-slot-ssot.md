# Hako Alloc Segment Arena Backing Modeled Arena Slot SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Record a modeled arena-slot inventory row from an accepted MIMAP-252A modeled
residence arena-binding report without creating real raw pointer residence,
pointer-derived lookup, or real arena backing.

The arena slot is a scalar/model inventory fact. It is not a runtime address,
not an allocation handle, not a pointer lookup key, and not an arena backing
allocation.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_arena_slot_box.hako
```

The owner may:

- observe accepted modeled residence arena-binding reports;
- preserve segment id, arena id, lifetime generation, residence token, and
  binding token;
- preserve scalar geometry and slot facts;
- reject missing/rejected inputs, invalid binding/residence tokens, invalid
  geometry, invalid slot shape, and closed-substrate requirements.

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
| `0` | modeled arena slot accepted |
| `1` | modeled residence arena-binding report missing |
| `2` | modeled residence arena-binding report was rejected |
| `3` | binding token invalid |
| `4` | residence token invalid |
| `5` | arena geometry invalid |
| `6` | arena slot shape invalid |
| `7` | closed-substrate requirement present |

## Validation

MIMAP-256A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh --level L2
```

The guard must:

- prove accepted modeled arena-slot publication;
- prove missing/rejected binding rejection;
- prove invalid binding token, residence token, geometry, slot shape, and
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
