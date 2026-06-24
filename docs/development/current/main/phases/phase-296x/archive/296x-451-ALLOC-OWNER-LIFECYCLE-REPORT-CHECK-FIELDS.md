---
Status: Done
Date: 2026-06-06
Scope: add AllocOwner lifecycle report fields and fastmem-check gates without opening reclaim behavior.
Blocker: MIM-FMEM-018B
Related:
  - docs/development/current/main/phases/phase-296x/296x-450-ALLOC-OWNER-LIFECYCLE-STATE-MACHINE.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - tools/hako_check/fastmem_capability_inventory_impl.py
  - tools/hako_check/fastmem_check.py
---

# 296x-451 AllocOwner Lifecycle Report/Check Fields

## Purpose

`MIM-FMEM-018A` fixed the lifecycle model. This row makes that model visible in
`hako_check` inventory output and rejects unsafe lifecycle reports before any
thread-exit shadow counters or abandoned reclaim behavior are added.

## Decision

The inventory now reports the lifecycle fields from the `018A` schema:

```text
allocator_owner_lifecycle_state_machine
allocator_owner_generation_enabled
allocator_owner_id_kind
allocator_owner_id_repr
allocator_owner_slot_bits
allocator_owner_generation_bits
allocator_owner_zero_is_invalid

allocator_owner_active_count
allocator_owner_exiting_flush_count
allocator_owner_abandoned_count
allocator_owner_reclaimed_count
allocator_owner_invalid_transition_count

allocator_owner_stale_generation_count
allocator_owner_generation_bump_count
allocator_owner_reuse_without_generation_bump_count

allocator_thread_exit_observed_count
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
page_reclaimed_with_remote_candidates
remote_free_drain_supported
```

`remote_candidate_unhandled_reclaim_block_count` remains observation-only for
now. It documents why reclaim is blocked, but it is not itself a failure until
the remote-drain/reclaim rows define stronger behavior.

## Check Gates

`fastmem-check` now fails lifecycle profiles when:

```text
allocator_owner_lifecycle_state_machine != 1
allocator_owner_generation_enabled != 1
allocator_owner_id_kind != arena_owner
allocator_owner_id_repr != packed_u64_slot_generation
allocator_owner_slot_bits != 32
allocator_owner_generation_bits != 32
allocator_owner_zero_is_invalid != 1

allocator_owner_invalid_transition_count != 0
allocator_owner_stale_generation_count != 0
allocator_owner_reuse_without_generation_bump_count != 0
allocator_exiting_owner_page_claim_count != 0
allocator_abandoned_owner_local_free_count != 0
page_reclaimed_with_remote_candidates != 0

allocator_abandoned_reclaim_success_count > 0
and remote_free_drain_supported == 0
```

## Scope

Accepted in this row:

```text
inventory fields for lifecycle identity/state/thread-exit/abandoned/reclaim-block evidence
fastmem-check fail-fast gates for invalid lifecycle reports
good fixture coverage through existing fastmem inventory/schema smokes
bad lifecycle fixture coverage through existing owner-state check smoke
```

Left for later:

```text
MIM-FMEM-018C producer-side shadow lifecycle counters
MIM-FMEM-019 AtomicRemoteHead drain
MIM-FMEM-020 abandoned reclaim behavior
```

## Acceptance

Proof:

```bash
python3 -m py_compile \
  tools/hako_check/fastmem_capability_inventory_impl.py \
  tools/hako_check/fastmem_check.py

bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_alloc_owner_schema_smoke.sh
bash tools/hako_check/fastmem_alloc_owner_check_smoke.sh
```

## Stop Line

- no producer-side lifecycle shadow counters yet
- no abandoned reclaim behavior
- no remote-drain behavior change
- no product activation, hook install, global allocator claim, or winner claim
