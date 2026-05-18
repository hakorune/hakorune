# Hako Alloc Segment Map Modeled Consume Ledger Release SSOT

Status: accepted
Decision: accepted
Date: 2026-05-18

## Purpose

MIMAP-161A adds a scalar modeled release route at the segment-map modeled
consume ledger owner boundary.

The route proves:

```text
accepted explicit-ID readiness
  -> modeled consume ledger live token
  -> modeled ledger release report
```

It reuses the existing `HakoAllocSegmentAllocationModeledLedger` release
substrate and does not introduce a second release ledger.

## Owner

```text
lang/src/hako_alloc/memory/segment_map_accepted_readiness_modeled_consume_ledger_box.hako
```

The owner may:

- release one live modeled token through `releaseConsumedToken`;
- map existing modeled ledger release reasons into the segment-map
  consume-ledger boundary;
- expose scalar release counters and report facts;
- preserve the accepted / blocked / duplicate / stale consume diagnostics from
  MIMAP-158A.

The owner must not:

- execute real segment free;
- allocate arena backing;
- use raw pointer residence;
- perform real segment-map lookup or mutation;
- claim or unclaim an atomic bitmap;
- call page-source / OSVM seams;
- schedule or spawn workers;
- activate providers, hooks, host allocator replacement, or
  `#[global_allocator]`;
- add backend `.inc` app/name matchers.

## Reason Codes

The release boundary uses local scalar reason codes:

| Code | Meaning |
| ---: | --- |
| `0` | released modeled consume-ledger token |
| `1` | invalid token shape |
| `2` | modeled token not found |
| `3` | modeled token already released |
| `4` | unsupported substrate requested |

The `release_reason` field preserves the upstream modeled ledger release reason.

## Acceptance Shape

The proof app must expose at least:

```text
release_first=1,0,0,0,70007002,70,7,2,1,0,1,0,3,1
blocked=0,4,4,-1,70007002
rejects=1,2,3,4,1,2,3,4
release_counts=5,1,4,1,1,1,1,70007002,4,0
```

## Validation

MIMAP-161A is a daily L2 row:

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_modeled_consume_ledger_release_guard.sh
```

Representative L3 EXE evidence is deferred to the future release closeout pack
unless this row introduces a new backend route shape.

## Stop Lines

- No real segment free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
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
MIMAP-162A segment-map modeled consume ledger release closeout pack
```
