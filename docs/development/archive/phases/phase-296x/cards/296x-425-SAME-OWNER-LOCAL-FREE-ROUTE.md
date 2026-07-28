---
Status: Done
Date: 2026-06-05
Scope: promote replacement-front same-owner local-free route evidence without adding a new smoke script.
Blocker: MIM-FMEM-012
Related:
  - docs/development/current/main/phases/phase-296x/296x-424-ALLOC-OWNER-SHADOW-COUNTERS.md
  - tools/allocator/replacement_front_bins_templates.py
  - tools/hako_check/fastmem_capability_inventory.py
---

# 296x-425 Same-Owner Local-Free Route

## Purpose

`MIM-FMEM-011C` made owner/TLS shadow counters visible. This row promotes the
existing benchmark-front TLS page-arena same-owner free path into explicit
route evidence, without adding another one-off smoke script.

## Decision

```text
same_owner_free_local_route_enabled=1
replacement_front_same_owner_local_free_route=page_meta_owner_local_free
same_owner_free_local_push_count derives from replacement_front_same_thread_free_local_count_total

remote_owner_free_remote_push_count=0
atomic_remote_head_enabled=0
product_activation=0
```

This is still benchmark-front evidence. It proves that same-owner frees are
observable as local-free route pushes when the TLS page-arena front is selected.
It does not open remote `AtomicRemoteHead` behavior or product allocator
activation.

## Smoke Growth Brake

```text
new_smoke_script_added=0
existing_fastmem_owner_shadow_smoke_extended=1
```

Report-only rows should not keep growing the smoke surface. Behavior rows may
extend an existing FastMemory smoke when that is enough to protect the new
boundary.

## Acceptance

```text
replacement_front_owner_shadow_counters=1
same_owner_free_local_route_enabled=1
replacement_front_same_owner_local_free_route=page_meta_owner_local_free
page_owner_same_count=1000
page_owner_remote_count=0
same_owner_free_local_candidate_count=1000
same_owner_free_local_push_count=1000
same_owner_free_local_fallback_count=0
remote_owner_free_remote_push_count=0
atomic_remote_head_enabled=0
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
same_owner_local_free_route_evidence=1
remote_atomic_head=0
new_smoke_script_added=0
source_rewrite=0
product_activation=0
```

Next row:

```text
MIM-FMEM-013 AtomicRemoteHead plan
```

## Stop Line

- do not open remote `AtomicRemoteHead` push/drain in this row
- do not claim `.hako` source-level thread support from C pthread evidence
- do not add another report-only smoke script
- do not start source rewrite / migration tooling here
- do not activate product replacement, hooks, global allocator, or winner claim
