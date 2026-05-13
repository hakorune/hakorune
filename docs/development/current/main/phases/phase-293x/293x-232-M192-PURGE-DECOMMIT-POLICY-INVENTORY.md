# 293x-232 M192 Purge Decommit Policy Inventory

Status: Complete

## Purpose

M192 adds a small `.hako` policy owner for purge/decommit candidate
classification before any future page-source execution row. The goal is to make
the stop line explicit: empty retired pages can become purge candidates, but
this row does not decommit, unreserve, or release OSVM pages.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_policy_box.hako
```

The owner returns a read-only `HakoAllocPurgeDecision` from
`HakoAllocPurgePolicyInventory.classifyLocalPage(...)`.

M192 classifies:

```text
missing backing -> rejected
live page -> rejected
empty but not retired page -> rejected
empty retired page with backing -> eligible candidate
```

All execution booleans stay false:

```text
would_decommit = 0
would_unreserve = 0
would_release_osvm = 0
```

## Stop Lines

- Do not call `HakoAllocPageSourcePolicy`.
- Do not call OSVM decommit/unreserve/release.
- Do not mutate heap/page state.
- Do not change allocation, release, realloc, aligned, or huge behavior.
- Do not add provider activation, hooks, or process allocator replacement.
- Do not add env toggles or mutable allocator options.

## Acceptance

- `HakoAllocPurgePolicyInventory.classifyLocalPage(...)` returns stable
  structured decisions for missing-backing, live, not-retired, and eligible
  empty-retired cases.
- VM and pure-first EXE proof output match the decision matrix.
- The guard confirms no page-source or `.inc` matcher leak.
- M192 guard stays local-run / index-listed and is not added to quick/dev
  gates.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_purge_policy_inventory_guard.sh
```
