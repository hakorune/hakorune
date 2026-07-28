---
Status: Done
Date: 2026-06-06
Scope: drain owner-page AtomicRemoteHead before abandoned/reclaim behavior can open.
Blocker: MIM-FMEM-019
Related:
  - docs/development/current/main/phases/phase-296x/296x-452-ALLOC-OWNER-LIFECYCLE-SHADOW-COUNTERS.md
  - docs/development/current/main/phases/phase-296x/296x-427-ATOMIC-REMOTE-HEAD-PILOT.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - tools/allocator/replacement_front_bins_templates.py
---

# 296x-453 AtomicRemoteHead Drain

## Purpose

`MIM-FMEM-018C` made lifecycle observations visible but intentionally left
remote candidates as reclaim blockers. This row drains already-published
remote frees from an owner page during thread-exit flush, so later abandoned
reclaim work can depend on a clear precondition:

```text
remote candidates are handled before reclaim can succeed
```

## Decision

The diagnostic replacement-front producer now owns a cold thread-exit drain
for page-index entries:

```text
owner thread exits
  -> inspect owned active page-index entries
  -> drain remote_head into the page local free stack
  -> count remote_free_drain_count
  -> count allocator_thread_exit_local_free_drain_count
  -> mark owner inactive
  -> abandon page if live blocks remain
```

If remote candidates remain after the drain attempt, they are still counted as
unhandled reclaim blockers:

```text
remote_candidate_unhandled_reclaim_block_count
allocator_abandoned_reclaim_blocked_count
allocator_abandoned_reclaim_blocked_remote_count
```

## Behavior Boundary

Accepted:

```text
thread-exit remote-head drain
remote_free_drain_count evidence
allocator_thread_exit_local_free_drain_count evidence
no reclaim-with-remote-candidates
```

Still closed:

```text
abandoned reclaim success
owner slot reuse
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
  tools/hako_check/fastmem_capability_inventory_impl.py \
  tools/hako_check/fastmem_check.py

bash tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
```

Additional manual compile/run probe:

```text
owner thread allocates a block, another thread frees it remotely while the
owner is still alive, then owner exits. The generated diagnostic front reports:

replacement_front_remote_free_drain_count>0
replacement_front_allocator_thread_exit_local_free_drain_count>0
replacement_front_remote_candidate_unhandled_reclaim_block_count=0
replacement_front_page_reclaimed_with_remote_candidates=0
```

## Stop Line

- do not reclaim abandoned pages in this row
- do not reuse owner slots
- do not claim product allocator activation

Next row:

```text
MIM-FMEM-020 abandoned reclaim
```
