---
Status: Done
Date: 2026-06-05
Scope: add fastmem-check fail-fast gates for AllocOwnerId / TLS owner-state reports.
Blocker: MIM-FMEM-011B
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-421-ALLOC-OWNER-ID-SCHEMA.md
  - tools/hako_check/fastmem_check.py
---

# 296x-423 AllocOwnerId Check Gates

## Purpose

`MIM-FMEM-011A` made AllocOwnerId / TLS owner-state fields visible. This row
makes bad owner-state reports fail fast before generated C shadow counters or
same-owner/remote-free behavior are added.

## Decision

```text
owner_state_check_gates=1
owner_state_profile=alloc_owner_id_capability|worker_id_capability|page_owner_check_enabled

worker_id_kind_required=allocator_arena_owner
worker_id_equals_os_thread_id_claim=0
worker_id_equals_runtime_worker_id_claim=0
worker_id_equals_hako_task_id_claim=0
hako_source_thread_support_claim=0

allocator_tls_arena_init_count>0
allocator_tls_arena_init_fail_count=0
page_owner_check_route=page_meta_owner_worker_id
page_owner_count_mismatch=0
page_owner_stale_generation_count=0
```

## Scope

Accepted in this row:

```text
fastmem-check rejects owner-state identity claim leaks
fastmem-check rejects TLS init failure / missing init
fastmem-check rejects page owner count mismatch and stale generation
fastmem-check rejects source-level thread support claims in owner-state reports
dedicated smoke fixes good and bad owner-state behavior
```

Left for later:

```text
generated C replacement-front owner shadow counters
same-owner local-free route
remote AtomicRemoteHead
thread-exit / abandoned owner lifecycle
product activation / hooks / global allocator claim
```

## Acceptance

Good report:

```text
alloc_owner_id_kind=allocator_arena_owner
worker_id_kind=allocator_arena_owner
allocator_tls_arena_init_count>0
page_owner_check_route=page_meta_owner_worker_id
page_owner_count_mismatch=0
summary=ok
```

Bad inventory:

```text
worker_id_equals_os_thread_id_claim=1
worker_id_equals_runtime_worker_id_claim=1
page_owner_count_mismatch=1
page_owner_stale_generation_count=1
hako_source_thread_support_claim=1
alloc_owner_id_kind=os_thread_id
worker_id_kind=os_thread_id
allocator_tls_arena_init_count=0
page_owner_check_route=owner_thread_id
summary=failed
```

Proof:

```bash
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_alloc_owner_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
alloc_owner_id_check_gates=1
bad_owner_state_inventory_rejected=1
generated_c_shadow_counters=0
same_owner_route_switch=0
remote_atomic_head=0
```

Next row:

```text
MIM-FMEM-011C replacement-front owner shadow counters
```

## Stop Line

- no generated-C owner shadow counters yet
- no same-owner local-free route switch
- no remote AtomicRemoteHead
- no thread support claim from C pthread evidence
- no product activation, hooks, global allocator claim, or winner claim
