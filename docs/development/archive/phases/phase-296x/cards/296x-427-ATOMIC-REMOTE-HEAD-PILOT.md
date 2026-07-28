---
Status: Done
Date: 2026-06-05
Scope: connect AtomicRemoteHead remote push/drain smoke evidence to fastmem inventory/check without product activation.
Blocker: MIM-FMEM-014
Related:
  - docs/development/current/main/phases/phase-296x/296x-426-ATOMIC-REMOTE-HEAD-PLAN.md
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
  - tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
---

# 296x-427 AtomicRemoteHead Pilot

## Purpose

`MIM-FMEM-013` made the remote-free plan vocabulary visible while keeping
remote behavior closed. This row promotes existing cross-thread
replacement-front smoke evidence into the FastMemory inventory/check surface.

## Decision

The pilot reads the existing non-activating cross-thread smoke pack fields:

```text
replacement_front_cross_thread_free_smoke_ok=1
replacement_front_cross_thread_free_policy=remote_queue
replacement_front_cross_thread_free_remote_free_push_count>0
replacement_front_cross_thread_free_remote_free_drain_count>0
replacement_front_cross_thread_free_arena_registry_overflow_count=0
```

and exposes them as the AtomicRemoteHead pilot fields:

```text
atomic_remote_head_plan=1
atomic_remote_head_route=page_remote_head_cas
atomic_remote_head_pilot_enabled=1
atomic_remote_head_enabled=1
remote_owner_free_remote_candidate_count>0
remote_owner_free_remote_push_count>0
remote_free_push_count>0
remote_free_drain_count>0
remote_free_memory_order=acq_rel
```

This is still benchmark-front evidence only.

## Boundary

```text
benchmark_thread_origin=c_pthread
hako_source_thread_support_claim=0
type_abi_hot_path_lookup_count=0
provider_dispatch_hot_path=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

No `.hako` source thread support is claimed from C pthread evidence. No
product allocator replacement is opened.

## Smoke Growth Brake

```text
new_smoke_script_added=0
existing_fastmem_owner_shadow_smoke_extended=1
new_fixture_added=atomic_remote_head_pilot_report.kv
```

The row extends the existing FastMemory owner-shadow smoke instead of adding a
new report-only smoke script.

## Acceptance

```text
atomic_remote_head_pilot_enabled=1
atomic_remote_head_enabled=1
remote_owner_free_remote_candidate_count=10
remote_owner_free_remote_push_count=10
remote_owner_free_fallback_lock_count=0
remote_free_push_count=10
remote_free_drain_count=10
replacement_front_cross_thread_free_arena_registry_overflow_count=0
summary=ok
```

Proof:

```bash
python3 -m py_compile tools/hako_check/fastmem_capability_inventory.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
atomic_remote_head_pilot_evidence=1
remote_push_behavior_observed=1
remote_drain_behavior_observed=1
source_rewrite=0
product_activation=0
```

Next row:

```text
MIM-FMEM-015 safe capability wrapper plan
```

## Stop Line

- do not claim `.hako` source-level thread support from C pthread evidence
- do not activate product replacement, hooks, global allocator, or winner claim
- do not turn FastMemory into a general raw pointer surface
- do not add more report-only smoke scripts for this row
