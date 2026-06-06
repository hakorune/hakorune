---
Status: Done
Date: 2026-06-06
Scope: open conservative abandoned reclaim after owner lifecycle and AtomicRemoteHead drain evidence.
Blocker: MIM-FMEM-020
Related:
  - docs/development/current/main/phases/phase-296x/296x-453-ATOMIC-REMOTE-HEAD-DRAIN.md
  - docs/development/current/main/phases/phase-296x/296x-450-ALLOC-OWNER-LIFECYCLE-STATE-MACHINE.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - tools/allocator/replacement_front_bins_templates.py
---

# 296x-454 Abandoned Reclaim

## Purpose

`MIM-FMEM-020` opens the first abandoned reclaim behavior after the
AllocOwner lifecycle and AtomicRemoteHead drain rows are in place.

This row does not make dead TLS-backed pages reusable by another thread. The
current diagnostic replacement-front producer still uses TLS page storage, so
cross-owner backing transfer remains closed. The accepted behavior here is the
safe lifecycle transition:

```text
owner exits
  -> drain remote_head
  -> if all owned page-index entries are empty and no remote candidates remain
     -> mark the abandoned owner as Reclaimed
     -> bump the owner generation
```

## Decision

Reclaim succeeds only when both preconditions hold:

```text
allocator_abandoned_live_page_count=0
remote_candidate_unhandled_reclaim_block_count=0
```

When they hold, the diagnostic front reports:

```text
allocator_abandoned_reclaim_attempt_count += empty abandoned page entries
allocator_abandoned_reclaim_success_count += empty abandoned page entries
allocator_owner_reclaimed_count += 1
allocator_owner_generation_bump_count += 1
page_reclaimed_with_remote_candidates=0
```

If live page entries remain, the owner stays abandoned. If remote candidates
remain after the drain attempt, reclaim is blocked and
`page_reclaimed_with_remote_candidates` must stay zero.

## Behavior Boundary

Accepted:

```text
empty abandoned owner-page index reclaim
generation bump evidence
remote-drain precondition enforcement
producer-neutral reclaim report/check evidence
```

Still closed:

```text
cross-owner reuse of dead TLS page backing
owner slot reuse as active owner
segment/product page backing transfer
product activation
hook install
global allocator claim
winner claim
full hako mimalloc algorithm claim
```

## Acceptance

Proof:

```bash
python3 -m py_compile \
  tools/allocator/replacement_front_bins_report_source.py \
  tools/allocator/replacement_front_bins_templates.py \
  tools/hako_check/replacement_front_report.py \
  tools/hako_check/fastmem_capability_inventory_impl.py \
  tools/hako_check/fastmem_check.py

bash tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
```

Additional manual compile/run probe:

```text
owner thread allocates a block, another thread frees it remotely while the
owner is still alive, then owner exits. The generated diagnostic front reports:

replacement_front_cross_thread_free_remote_push_count=1
replacement_front_remote_free_drain_count=1
replacement_front_allocator_abandoned_reclaim_success_count>0
replacement_front_allocator_owner_reclaimed_count=1
replacement_front_allocator_owner_generation_bump_count=1
replacement_front_remote_candidate_unhandled_reclaim_block_count=0
replacement_front_page_reclaimed_with_remote_candidates=0
```

## Stop Line

- do not attach reclaimed TLS backing to another owner
- do not reuse owner slots as active owner in this row
- do not claim product allocator activation
- do not remove the Python-template C diagnostic baseline from this row

Next row:

```text
MIR-FMEM-008 replacement-front layout/table/owner runtime producer selection
```
