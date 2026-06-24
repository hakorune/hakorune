---
Status: Done
Date: 2026-06-05
Scope: split MIM-FMEM-011 into AllocOwnerId/TLS owner-state schema, checks, and shadow-counter implementation rows.
Blocker: MIM-FMEM-011
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-419-TYPED-PAGE-META-HANDLE-PLAN.md
---

# 296x-420 AllocOwnerId TLS Owner-State Task Split

## Purpose

`MIM-FMEM-010` made `TypedPageMetaHandle` report evidence visible. The next
boundary is owner identity. The term `WorkerId` is too easy to confuse with OS
threads, C pthread benchmark threads, `.hako` runtime workers, and source-level
`nowait/task_scope` semantics.

This card makes `AllocOwnerId` the design subject and splits `MIM-FMEM-011`
into smaller rows.

## Decision

```text
canonical_identity_name=AllocOwnerId
compat_report_prefix=worker_id_*
worker_id_kind=allocator_arena_owner

worker_id_equals_os_thread_id_claim=0
worker_id_equals_runtime_worker_id_claim=0
worker_id_equals_hako_task_id_claim=0

benchmark_thread_origin=c_pthread
hako_source_thread_support_claim=0
```

`AllocOwnerId` is allocator-local TLS arena / page ownership identity. It is not
an OS thread id, runtime worker id, or `.hako` task id. C pthread benchmark
threads are only the host execution source that causes allocator TLS arenas to
exist.

## Task Split

### MIM-FMEM-011A: AllocOwnerId / TLS Owner-State Schema

Scope:

```text
docs/report vocabulary only
AllocOwnerIdV0
TlsArenaOwnerStateV0
PageOwnerCheckV0
compat worker_id_* fields
```

Acceptance:

```text
alloc_owner_id_capability=1
alloc_owner_id_kind=allocator_arena_owner
worker_id_kind=allocator_arena_owner
worker_id_equals_os_thread_id_claim=0
worker_id_equals_runtime_worker_id_claim=0
worker_id_equals_hako_task_id_claim=0
benchmark_thread_origin=c_pthread
hako_source_thread_support_claim=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

### MIM-FMEM-011B: fastmem-check Owner-State Gates

Scope:

```text
fail-fast checks over existing report/inventory fields
owner kind / no-escape checks
TLS arena init failure checks
page-owner count consistency checks
boundary claim checks
```

Acceptance:

```text
worker_id_escape_count=0
allocator_tls_arena_init_fail_count=0
page_owner_check_count == same + remote + unowned + stale + invalid
page_owner_stale_generation_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
hako_source_thread_support_claim=0
```

### MIM-FMEM-011C: Replacement-Front Owner Shadow Counters

Scope:

```text
generated C replacement-front shadow evidence
current AllocOwnerId read/init
TLS arena init/live/peak counters
PageMeta.owner_worker_id assignment evidence
same/remote/unowned/stale/invalid owner comparison counters
```

Acceptance:

```text
page_owner_check_enabled=1
page_owner_check_route=page_meta_owner_worker_id
page_owner_check_count>0 for mixed-ws/free-path profile
same_owner_free_local_candidate_count observable
remote_owner_free_remote_candidate_count observable
remote owner never enters local_free
```

Still closed:

```text
remote_head CAS push
same-owner local-free route switch
thread-exit abandoned reclaim behavior
product activation
hook install
global allocator claim
winner claim
full .hako mimalloc algorithm claim
```

## Next Rows

```text
MIM-FMEM-012:
  same-owner local-free route

MIM-FMEM-013:
  AtomicRemoteHead plan

MIM-FMEM-014:
  AtomicRemoteHead pilot

MIM-FMEM-015:
  thread-exit / abandoned owner lifecycle
```

## Stop Line

- do not use OS thread id as allocator owner truth
- do not use `.hako` task or runtime worker identity as allocator owner truth
  without an explicit mapping row
- do not claim source-level thread support from C pthread benchmark evidence
- do not route remote-owner frees into local_free
- do not activate product replacement, hooks, global allocator, or winner claim
