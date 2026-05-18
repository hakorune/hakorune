# Hako Alloc Segment Arena Backing Modeled Source Accounting SSOT

Status: active
Date: 2026-05-19
Decision: accepted

## Purpose

Record scalar/model accounting over accepted MIMAP-260A modeled source bridge
reports before real arena backing allocation opens.

The accounting row captures source capacity, committed bytes, uncommitted
bytes, padded bytes, and slot capacity as model facts. It is not a runtime
memory provider, not an OSVM/page-source call, not a pointer lookup key, and
not an arena allocation.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_accounting_box.hako
```

The owner may:

- observe accepted modeled source bridge reports;
- preserve segment id, arena id, source token, and arena-slot token;
- compute source uncommitted bytes and available bytes after the padded slot;
- reject missing/rejected source bridge reports, invalid source token, invalid
  accounting geometry, and closed-substrate requirements.

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
| `0` | modeled source accounting accepted |
| `1` | modeled source bridge report missing |
| `2` | modeled source bridge report was rejected |
| `3` | source token invalid |
| `4` | source accounting geometry invalid |
| `5` | closed-substrate requirement present |

## Validation

MIMAP-264A uses daily L2 validation:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_accounting_guard.sh --level L2
```

The guard must:

- prove accepted modeled source accounting publication;
- prove missing/rejected source bridge rejection;
- prove invalid source token, invalid accounting geometry, and
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
