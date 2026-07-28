---
Status: Done
Date: 2026-06-05
Scope: add AllocOwnerId / TLS arena owner-state schema to fastmem capability inventory without behavior changes.
Blocker: MIM-FMEM-011A
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-420-ALLOC-OWNER-ID-TLS-OWNER-STATE-TASK-SPLIT.md
  - tools/hako_check/fastmem_capability_inventory.py
---

# 296x-421 AllocOwnerId Schema

## Purpose

`MIM-FMEM-011A` makes owner-state vocabulary visible before owner-state
fail-fast checks or replacement-front shadow-counter implementation. The report
now distinguishes allocator arena ownership from OS threads, runtime workers,
and `.hako` task identity.

## Decision

```text
alloc_owner_id_capability=1
alloc_owner_id_kind=allocator_arena_owner
alloc_owner_id_source=benchmark_c_pthread_tls
alloc_owner_id_width_bits=64
alloc_owner_id_generation_enabled=1

worker_id_kind=allocator_arena_owner
worker_id_equals_os_thread_id_claim=0
worker_id_equals_runtime_worker_id_claim=0
worker_id_equals_hako_task_id_claim=0

benchmark_thread_origin=c_pthread
hako_source_thread_support_claim=0
```

## Scope

Accepted in this row:

```text
fastmem capability inventory exposes AllocOwnerId schema fields
compat worker_id_* fields are tied to allocator_arena_owner semantics
TLS arena owner-state schema fields are visible
page owner-check schema fields are visible
dedicated schema smoke fixes the report contract
```

Left for later:

```text
fastmem-check owner-state fail-fast gates
generated C replacement-front owner shadow counters
same-owner local-free route
AtomicRemoteHead
thread-exit / abandoned owner lifecycle
product activation / hooks / global allocator claim
```

## Acceptance

```text
alloc_owner_id_capability=1
alloc_owner_id_kind=allocator_arena_owner
worker_id_kind=allocator_arena_owner
worker_id_equals_os_thread_id_claim=0
worker_id_equals_runtime_worker_id_claim=0
worker_id_equals_hako_task_id_claim=0
allocator_tls_arena_enabled=1
allocator_tls_arena_mode=benchmark_c_tls
page_owner_check_enabled=1
page_owner_check_route=page_meta_owner_worker_id
page_owner_count_mismatch=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

Proof:

```bash
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_alloc_owner_schema_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
alloc_owner_id_schema_surface=1
worker_id_compat_kind_fixed=allocator_arena_owner
source_thread_support_claim=0
behavior_change=0
```

Next row:

```text
MIM-FMEM-011B fastmem-check owner-state gates
```

## Stop Line

- no generated-C owner shadow counters yet
- no same-owner local-free route switch
- no remote `AtomicRemoteHead`
- no thread support claim from C pthread evidence
- no product activation, hooks, global allocator claim, or winner claim
