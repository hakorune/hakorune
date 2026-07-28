---
Status: Done
Date: 2026-06-05
Scope: normalize generated-C replacement-front owner shadow counters into AllocOwnerId / TLS owner-state evidence.
Blocker: MIM-FMEM-011C
Related:
  - docs/development/current/main/phases/phase-296x/296x-420-ALLOC-OWNER-ID-TLS-OWNER-STATE-TASK-SPLIT.md
  - docs/development/current/main/phases/phase-296x/296x-423-ALLOC-OWNER-ID-CHECK-GATES.md
  - tools/hako_check/fastmem_capability_inventory.py
---

# 296x-424 AllocOwnerId Shadow Counters

## Purpose

`MIM-FMEM-011B` made bad owner-state reports fail fast. This row connects the
benchmark-only generated-C replacement front counters to that owner-state
surface, without changing allocator behavior.

## Decision

```text
replacement_front_owner_shadow_counters=1
owner_counter_source=replacement_front_owner_thread_id_*_count_total
tls_counter_source=replacement_front_tls_arena_*_count_total

alloc_owner_id_kind=allocator_arena_owner
alloc_owner_id_source=benchmark_c_pthread_tls
page_owner_check_route=page_meta_owner_worker_id

same_owner_free_local_candidate_count observable
remote_owner_free_remote_candidate_count observable
same_owner_free_local_push_count=0
remote_owner_free_remote_push_count=0
```

The generated C front currently publishes a monotonic TLS owner token. It is
allocator-local evidence, not OS thread id / runtime worker id / `.hako` task
id. It does not yet claim a generation-bearing owner id unless the report
explicitly says so.

## Scope

Accepted in this row:

```text
replacement_front_report reads owner same-count and TLS arena total counters
fastmem inventory derives AllocOwnerId evidence from generated-C shadow counters
fastmem inventory derives page-owner same/remote candidates
fastmem-check passes owner-shadow reports without explicit owner schema fields
```

Left for later:

```text
same-owner local-free route switch
remote AtomicRemoteHead push/drain
thread-exit / abandoned owner lifecycle
product activation / hooks / global allocator claim / winner claim
full .hako mimalloc algorithm claim
```

## Acceptance

Input report with only replacement-front shadow counters:

```text
subject_N_replacement_front_owner_thread_id_lookup_count_total=1000
subject_N_replacement_front_owner_thread_id_same_count_total=990
subject_N_replacement_front_owner_thread_id_remote_count_total=10
subject_N_replacement_front_tls_arena_count_total=2
subject_N_replacement_front_tls_arena_peak_count_total=2
```

Inventory/check output:

```text
replacement_front_owner_shadow_counters=1
alloc_owner_id_capability=1
alloc_owner_id_kind=allocator_arena_owner
alloc_owner_id_source=benchmark_c_pthread_tls
allocator_tls_arena_init_count=2
allocator_tls_arena_peak_count=2
page_owner_check_enabled=1
page_owner_check_route=page_meta_owner_worker_id
page_owner_check_count=1000
page_owner_same_count=990
page_owner_remote_count=10
same_owner_free_local_push_count=0
remote_owner_free_remote_push_count=0
summary=ok
```

Proof:

```bash
bash tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
bash tools/hako_check/fastmem_alloc_owner_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
owner_shadow_counter_inventory_bridge=1
generated_c_behavior_change=0
same_owner_route_switch=0
remote_atomic_head=0
product_activation=0
```

Next row:

```text
MIM-FMEM-012 same-owner local-free route
```

## Stop Line

- do not route same-owner frees to local_free in this row
- do not route remote-owner frees to AtomicRemoteHead in this row
- do not claim source-level thread support from C pthread evidence
- do not claim full `.hako` mimalloc algorithm coverage
- do not activate product replacement, hooks, global allocator, or winner claim
