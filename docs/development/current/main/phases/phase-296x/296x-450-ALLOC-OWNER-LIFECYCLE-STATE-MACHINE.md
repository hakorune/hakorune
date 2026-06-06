---
Status: Done
Date: 2026-06-06
Scope: document AllocOwnerId page ownership lifecycle truth before adding report/check fields or reclaim behavior.
Blocker: MIM-FMEM-018A
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-421-ALLOC-OWNER-ID-SCHEMA.md
  - docs/development/current/main/phases/phase-296x/296x-423-ALLOC-OWNER-ID-CHECK-GATES.md
  - docs/development/current/main/phases/phase-296x/296x-424-ALLOC-OWNER-SHADOW-COUNTERS.md
---

# 296x-450 AllocOwner Lifecycle State Machine

## Purpose

`MIM-FMEM-018` is not primarily "thread exit cleanup". It fixes the
`AllocOwnerId` ownership lifecycle truth that makes thread exit, abandoned
pages, stale owner detection, and later reclaim safe.

This row is the documentation split for that boundary. Report/check fields and
shadow evidence open in the next row.

## Decision

Persistent owner/page lifecycle states:

```text
Active
  -> ExitingFlush
  -> Abandoned
  -> Reclaimed
```

`ReclaimAttempt` is a transient cold-path operation, not a persistent hot-path
state.

`AllocOwnerId` is generation-bearing from v0:

```text
allocator_owner_id_repr=packed_u64_slot_generation
allocator_owner_slot_bits=32
allocator_owner_generation_bits=32
allocator_owner_generation_enabled=1
allocator_owner_zero_is_invalid=1
```

Hot paths compare the packed `u64` owner id directly. Slot/generation splitting
is for cold-path lifecycle checks, stale detection, report interpretation, and
debugging.

## Invariants

Owner identity:

```text
Active owner is the only owner state allowed to claim new pages.
ExitingFlush owner must not claim new pages.
Abandoned owner must not be treated as a same-owner local_free owner.
Reclaimed owner slot must not be reused without a generation bump.
```

Page ownership:

```text
page.owner_id is stable during an Active lifetime.

Allowed transitions:
  Active -> ExitingFlush -> Abandoned
  Active -> ExitingFlush -> Reclaimed
  Abandoned -> Reclaimed

Disallowed transitions are report/check failures.
```

Free/reclaim safety:

```text
same owner:
  local_free candidate

remote owner:
  remote_head candidate or conservative fallback

exiting / abandoned / stale owner:
  never enter local_free

unhandled remote candidates:
  block reclaim

page_reclaimed_with_remote_candidates:
  must stay 0
```

## Report Schema To Add Next

`MIM-FMEM-018B` should add these fields to the inventory/check surface:

```text
allocator_owner_lifecycle_state_machine=1
allocator_owner_generation_enabled=1

allocator_owner_id_kind=arena_owner
allocator_owner_id_repr=packed_u64_slot_generation
allocator_owner_slot_bits=32
allocator_owner_generation_bits=32
allocator_owner_zero_is_invalid=1

allocator_owner_active_count
allocator_owner_exiting_flush_count
allocator_owner_abandoned_count
allocator_owner_reclaimed_count
allocator_owner_invalid_transition_count=0

allocator_owner_stale_generation_count=0
allocator_owner_generation_bump_count
allocator_owner_reuse_without_generation_bump_count=0

allocator_thread_exit_observed_count
allocator_thread_exit_flush_supported=0|1
allocator_thread_exit_flush_count
allocator_thread_exit_flush_page_count
allocator_thread_exit_local_free_drain_count
allocator_thread_exit_remote_candidate_seen_count

allocator_abandoned_owner_count
allocator_abandoned_page_count
allocator_abandoned_live_page_count
allocator_abandoned_empty_page_count
allocator_abandoned_remote_candidate_count

allocator_abandoned_reclaim_attempt_count
allocator_abandoned_reclaim_success_count
allocator_abandoned_reclaim_blocked_count
allocator_abandoned_reclaim_blocked_remote_count

remote_candidate_unhandled_reclaim_block_count
page_reclaimed_with_remote_candidates=0
```

Compatibility note:

```text
alloc_owner_id_* and worker_id_* remain compatibility/report aliases for the
MIM-FMEM-011 owner-state surface. New lifecycle fields should use
allocator_owner_* unless an existing field already has a stable name.
```

Boundary proof remains unchanged:

```text
benchmark_thread_origin=c_pthread
hako_source_thread_support_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
hako_mimalloc_algorithm_claim=0
replacement_front_is_full_hako_algorithm=0
```

## Fail-Fast To Add Next

`MIM-FMEM-018B` should fail lifecycle profiles when:

```text
allocator_owner_lifecycle_state_machine != 1
allocator_owner_generation_enabled != 1
allocator_owner_invalid_transition_count != 0
allocator_owner_stale_generation_count != 0
allocator_owner_reuse_without_generation_bump_count != 0

allocator_exiting_owner_page_claim_count != 0
allocator_abandoned_owner_local_free_count != 0
page_reclaimed_with_remote_candidates != 0

allocator_abandoned_reclaim_success_count > 0
and remote_free_drain_supported == 0
```

`remote_candidate_unhandled_reclaim_block_count > 0` is observe/warn until
`AtomicRemoteHead` drain is fully available. It must still block reclaim.

## Scope

Accepted in this row:

```text
MIM-FMEM-018 split into lifecycle docs, report/check fields, shadow evidence,
remote drain, and abandoned reclaim rows
Active / ExitingFlush / Abandoned / Reclaimed state machine fixed
ReclaimAttempt classified as transient
generation-bearing AllocOwnerId fixed as lifecycle truth
reclaim-with-remote-candidates banned
```

Left for later:

```text
MIM-FMEM-018B report/check fields
MIM-FMEM-018C shadow lifecycle counters
MIM-FMEM-019 AtomicRemoteHead drain
MIM-FMEM-020 abandoned reclaim
product activation / hook install / global allocator claim / winner claim
```

## Acceptance

Documentation acceptance:

```text
MIM-FMEM-018A landed as a docs-only lifecycle split
current pointer moves to MIM-FMEM-018B
no reclaim behavior opened
no new smoke script added
```

Proof:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

- do not implement abandoned reclaim in this row
- do not make `ReclaimAttempt` a persistent hot-path state
- do not reuse owner slots without generation
- do not route remote / stale / abandoned owner frees to local_free
- do not claim `.hako` source-level thread support from C pthread evidence
- do not activate product replacement, hooks, global allocator, or winner claim
