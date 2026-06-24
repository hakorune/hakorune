---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-035.
Related:
  - docs/development/current/main/phases/phase-296x/296x-532-MIM-PORT-FMEM-034-ATOMIC-REMOTE-HEAD-ROUTE-DRAIN-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-531-MIM-PORT-FMEM-033-ATOMIC-REMOTE-HEAD-CAS-LOWERING-PRODUCER-PILOT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-533 MIM-PORT-FMEM-035 AtomicRemoteHead Retry Policy Preflight

## Purpose

Add report/check evidence for the retry policy that must wrap
`AtomicRemoteHeadPush` before drain/exchange or remote-owner branch routing is
opened.

MIM-033 emits one `cmpxchg` attempt. MIM-035 does not change that lowering yet.
It pins the next producer contract so the retry implementation row has a clear
acceptance surface.

## Required Report Shape

For the `.hako` AtomicRemoteHead pilot:

```text
replacement_front_selected_memop_family=remote_free
replacement_front_selected_memop_kinds=AtomicRemoteHeadPush
fastmem_atomic_remote_head_retry_preflight=1
atomic_remote_head_retry_policy_selected=1
atomic_remote_head_retry_policy_open=0
atomic_remote_head_retry_attempt_limit=<positive integer>
atomic_remote_head_retry_lowered_count=0
atomic_remote_head_cas_lowering_open=1
memop_atomic_remote_head_lowered_count=1
atomic_remote_head_push_lowerable_count=1
atomic_remote_head_remote_owner_missing_count=0
atomic_remote_head_block_next_missing_count=0
atomic_remote_head_memory_order_policy=acq_rel
```

## Must Fail

`fastmem-check` must reject retry-preflight reports that:

```text
miss the retry-policy selection marker
claim retry lowering is already open
set retry_attempt_limit <= 0
drop the MIM-033 CAS producer evidence
miss remote-owner or block-next proofs
open drain/exchange
open remote-owner branch routing
open TLS backing transfer
open activation / hooks / global allocator claim / winner claim
```

## Still Closed

```text
retry loop LLVM lowering
CAS result branch routing
AtomicRemoteHead drain/exchange
remote-owner branch routing
TLS backing transfer
owner slot reuse
process allocator replacement
hook installation
global allocator claim
winner claim
full .hako mimalloc algorithm claim
```

## Acceptance

```text
fastmem_mir_to_llvm_producer_report.py can emit the retry preflight profile
fastmem-check accepts the good retry-preflight fixture
fastmem-check rejects a bad retry-preflight fixture
fastmem_source_syntax_smoke covers the retry preflight report/check path
MIM-033 direct LLVM object emission remains green
product_activation=0
global_allocator_claim=0
winner_claim=0
```

## Landed

The retry policy is now visible as a producer-neutral report/check preflight:

```text
--profile remote-free-retry-preflight

fastmem_atomic_remote_head_retry_preflight=1
atomic_remote_head_retry_policy_selected=1
atomic_remote_head_retry_policy_open=0
atomic_remote_head_retry_attempt_limit=3
atomic_remote_head_retry_lowered_count=0
atomic_remote_head_drain_open=0
remote_owner_branch_routing_open=0
atomic_remote_head_cas_lowering_open=1
memop_atomic_remote_head_lowered_count=1
```

The MIM-033 single-attempt CAS producer evidence remains required and positive.
Retry lowering, drain/exchange, branch routing, TLS transfer, and activation
remain closed.

## Verification

```text
python3 -m py_compile tools/hako_check/fastmem_mir_to_llvm_producer_report.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIM-PORT-FMEM-036:
  AtomicRemoteHead retry lowering producer pilot.

Scope:
  retry loop over the existing verified AtomicRemoteHeadPush CAS material.

Still closed:
  drain/exchange
  remote-owner branch routing
  TLS backing transfer
  product activation / hook / global allocator claim / winner claim
```
