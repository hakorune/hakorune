---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-036.
Related:
  - docs/development/current/main/phases/phase-296x/296x-533-MIM-PORT-FMEM-035-ATOMIC-REMOTE-HEAD-RETRY-POLICY-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-531-MIM-PORT-FMEM-033-ATOMIC-REMOTE-HEAD-CAS-LOWERING-PRODUCER-PILOT.md
  - src/llvm_py/instructions/memop.py
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
---

# 296x-534 MIM-PORT-FMEM-036 AtomicRemoteHead Retry Lowering Producer Pilot

## Purpose

Open the first bounded retry lowering for `AtomicRemoteHeadPush` after MIM-035
made the retry policy visible in report/check evidence.

This row may change the LLVM producer shape for `AtomicRemoteHeadPush`, but only
inside the already verified remote-free publication primitive.

## Candidate Lowering Shape

```text
AtomicRemoteHeadPush(page, block):
  for attempt in 0..retry_attempt_limit:
    old_head = atomic_load(remote_head, acquire)
    block.next = old_head
    result = cmpxchg(remote_head, old_head, block, acq_rel/acquire)
    if result.success:
      break
  expose retry-lowered evidence
```

The first producer pilot can be conservative. It does not need to route the
failure result to source-visible control flow yet, but it must not silently
claim drain/branch behavior.

## Still Closed

```text
AtomicRemoteHead drain/exchange
remote-owner branch routing
same/remote free full body route
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
atomic_remote_head_retry_policy_open=1
atomic_remote_head_retry_attempt_limit=3
atomic_remote_head_retry_lowered_count=1
memop_atomic_remote_head_lowered_count=1
atomic_remote_head_drain_open=0
remote_owner_branch_routing_open=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
global_allocator_claim=0
winner_claim=0
```
