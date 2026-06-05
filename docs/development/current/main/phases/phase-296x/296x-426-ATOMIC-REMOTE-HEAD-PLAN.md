---
Status: Done
Date: 2026-06-05
Scope: add AtomicRemoteHead plan vocabulary without opening remote push/drain behavior.
Blocker: MIM-FMEM-013
Related:
  - docs/development/current/main/phases/phase-296x/296x-425-SAME-OWNER-LOCAL-FREE-ROUTE.md
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
---

# 296x-426 AtomicRemoteHead Plan

## Purpose

`MIM-FMEM-012` promoted same-owner local-free route evidence. This row fixes
the remote-free vocabulary before opening remote push/drain behavior.

## Decision

```text
atomic_remote_head_plan=1
atomic_remote_head_route=page_remote_head_cas
remote_free_memory_order=acq_rel

atomic_remote_head_pilot_enabled=0
atomic_remote_head_enabled=0
remote_owner_free_remote_push_count=0
remote_free_drain_count=0
```

The plan describes the intended remote owner path. It does not execute remote
push/drain yet. The actual benchmark-front remote behavior remains for
`MIM-FMEM-014`.

## Smoke Growth Brake

```text
new_smoke_script_added=0
existing_fastmem_owner_shadow_smoke_extended=1
```

This row extends the existing owner shadow smoke instead of creating another
report-only smoke script.

## Acceptance

```text
atomic_remote_head_plan=1
atomic_remote_head_route=page_remote_head_cas
atomic_remote_head_pilot_enabled=0
atomic_remote_head_enabled=0
remote_free_memory_order=acq_rel
remote_owner_free_remote_push_count=0
summary=ok
```

Proof:

```bash
bash tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
atomic_remote_head_plan_vocabulary=1
remote_push_behavior=0
remote_drain_behavior=0
new_smoke_script_added=0
source_rewrite=0
product_activation=0
```

Next row:

```text
MIM-FMEM-014 AtomicRemoteHead pilot
```

## Stop Line

- do not enable remote push/drain behavior in this row
- do not add another report-only smoke script
- do not start source rewrite / migration tooling here
- do not claim `.hako` source-level thread support from C pthread evidence
- do not activate product replacement, hooks, global allocator, or winner claim
