# Hako Alloc Segment Arena Backing Modeled Residence Arena-Binding SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Bind an accepted MIMAP-248A modeled no-escape address residence report to an
accepted MIMAP-240A scalar requirement matrix report for the same segment and
arena without creating real raw pointer residence, pointer-derived lookup, or
real arena backing.

The binding is a scalar/model ledger fact. It is not a runtime pointer, not a
segment-map lookup key, and not an arena allocation handle.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_residence_arena_binding_box.hako
```

The owner may:

- observe accepted modeled no-escape address residence reports;
- observe accepted scalar requirement matrix reports;
- verify that segment id and arena id match;
- preserve residence token, lifetime generation, and arena geometry facts;
- reject missing/rejected inputs, mismatched ids, invalid tokens, invalid
  geometry, and closed-substrate requirements.

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
| `0` | modeled residence arena-binding accepted |
| `1` | residence report missing |
| `2` | residence report was rejected |
| `3` | requirement matrix report missing |
| `4` | requirement matrix report was rejected |
| `5` | segment id mismatch |
| `6` | arena id mismatch |
| `7` | residence token invalid |
| `8` | requirement matrix geometry invalid |
| `9` | closed-substrate requirement present |

## Validation

MIMAP-252A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_residence_arena_binding_guard.sh --level L2
```

The guard must:

- prove accepted modeled residence arena-binding publication;
- prove missing/rejected residence and matrix rejection;
- prove segment/arena mismatch, invalid token, invalid geometry, and
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
