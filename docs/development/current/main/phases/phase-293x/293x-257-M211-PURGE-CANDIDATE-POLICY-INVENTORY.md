# 293x-257 M211 Purge Candidate Policy Inventory

Status: Complete

## Purpose

M211 adds a read-only purge candidate policy inventory after the M207 lifecycle
vocabulary and M208/M209 observer surfaces are frozen.

The row consumes already-built lifecycle reports. It does not observe heap
pages, scan queues, schedule purge work, decommit, recommit, or mutate
allocator state.

## Decision

Decision: accepted.

Add:

```text
lang/src/hako_alloc/memory/purge_candidate_policy_box.hako
```

The owner returns a read-only `HakoAllocPurgeCandidateDecision` from
`HakoAllocPurgeCandidatePolicyInventory.classifyLifecycleReport(report)`.

M211 classifies:

```text
null report -> rejected
missing page/backing -> rejected
active page -> rejected
decommitted page -> rejected, recommit required
recommitted-active page -> rejected
retired but not decommit-candidate -> rejected
retired lifecycle decommit-candidate with backing -> eligible future candidate
```

All execution and scheduler booleans stay false:

```text
would_schedule_decommit = 0
would_decommit = 0
would_recommit = 0
would_unreserve = 0
would_release_osvm = 0
```

## Stop Lines

- Do not execute decommit or recommit.
- Do not schedule purge work.
- Do not scan heap queues.
- Do not call page-source APIs.
- Do not call M199/M205 attempt owners from the M211 owner.
- Do not call M207 `observeHeapPage(...)` from the M211 owner.
- Do not mutate heap/page/marker/page-source state.
- Do not unreserve or release OSVM pages.
- Do not add provider activation, hooks, env toggles, or allocator replacement.
- Do not add backend `.inc` app/name matchers.
- Do not alter existing allocator behavior.

## Acceptance

- `HakoAllocPurgeCandidatePolicyInventory.classifyLifecycleReport(...)`
  returns stable structured decisions for null, missing, active, retired-busy,
  retired-candidate, decommitted, and recommitted-active lifecycle reports.
- VM and pure-first EXE proof output match the decision matrix.
- The guard confirms no page-source, scheduler, observer-call, mutation, or
  `.inc` matcher leak.
- M211 guard stays local-run / index-listed and is not added to quick/dev gates.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_purge_candidate_policy_inventory_guard.sh
```

